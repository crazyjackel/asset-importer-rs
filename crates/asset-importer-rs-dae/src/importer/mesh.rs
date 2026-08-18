use std::collections::HashMap;

use asset_importer_rs_scene::{
    AI_MAX_NUMBER_OF_COLORS_SETS, AI_MAX_NUMBER_OF_TEXTURECOORDS, AiColor4D, AiMesh,
    AiPrimitiveType, AiReal, AiVector3D,
};
use dae_parser::{
    Controller, Document, Geometry, Instance, InstanceMaterial, LocalMap, Material,
    Mesh as DocumentMesh, Node, Primitive, Semantic, Source, Url,
    source::{SourceReader, XYZ},
};
use enumflags2::BitFlags;

use crate::DaeImportError;

use super::{DaeImporter, material::material_key};

/// Different types of input data to a vertex or face.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum InputType {
    #[default]
    Invalid,
    /// Per-index data referring to the `<vertices>` element.
    Vertex,
    Position,
    Normal,
    Texcoord,
    Color,
    Tangent,
    Bitangent,
}

impl InputType {
    fn from_semantic(semantic: &Semantic) -> Self {
        match semantic {
            Semantic::Vertex => Self::Vertex,
            Semantic::Position => Self::Position,
            Semantic::Normal => Self::Normal,
            Semantic::TexCoord | Semantic::UV => Self::Texcoord,
            Semantic::Color => Self::Color,
            Semantic::Tangent | Semantic::TexTangent => Self::Tangent,
            Semantic::Binormal | Semantic::TexBinormal => Self::Bitangent,
            _ => Self::Invalid,
        }
    }

    fn from_semantic_str(semantic: &str) -> Self {
        match semantic {
            "VERTEX" => Self::Vertex,
            "POSITION" => Self::Position,
            "NORMAL" => Self::Normal,
            "TEXCOORD" | "UV" => Self::Texcoord,
            "COLOR" => Self::Color,
            "TANGENT" | "TEXTANGENT" => Self::Tangent,
            "BINORMAL" | "TEXBINORMAL" => Self::Bitangent,
            _ => Self::Invalid,
        }
    }
}

/// An input channel for mesh data, referring to a single accessor/source.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct InputChannel {
    type_: InputType,
    index: usize,
    offset: usize,
    accessor: String,
}

/// Short vertex-index description for effect → mesh semantic mapping.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct InputSemanticMapEntry {
    set: u32,
    type_: InputType,
}

/// Table to map from effect to vertex input semantics.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct SemanticMappingTable {
    mat_name: String,
    map: HashMap<String, InputSemanticMapEntry>,
}

impl From<&InstanceMaterial> for SemanticMappingTable {
    fn from(material: &InstanceMaterial) -> Self {
        let mut map = HashMap::new();
        for bind in &material.bind_vertex_input {
            map.insert(
                bind.semantic.clone(),
                InputSemanticMapEntry {
                    set: bind.input_set.unwrap_or(0),
                    type_: InputType::from_semantic_str(&bind.input_semantic),
                },
            );
        }
        Self {
            mat_name: url_key(&material.target.val).to_string(),
            map,
        }
    }
}

/// A reference to a mesh inside a node, including materials assigned to subgroups.
#[derive(Clone, Debug, Default)]
struct MeshInstance {
    /// ID of the mesh or controller to be instanced.
    mesh_or_controller: String,
    /// Materials by the subgroup ID they're applied to, in document order.
    materials: Vec<(String, SemanticMappingTable)>,
}

impl MeshInstance {
    fn material_for_symbol(&self, symbol: &str) -> Option<&SemanticMappingTable> {
        self.materials
            .iter()
            .find(|(bound_symbol, _)| bound_symbol == symbol)
            .map(|(_, table)| table)
            .or_else(|| self.materials.first().map(|(_, table)| table))
    }
}

impl From<&Instance<Geometry>> for MeshInstance {
    fn from(instance: &Instance<Geometry>) -> Self {
        Self {
            mesh_or_controller: url_key(&instance.url.val).to_string(),
            materials: instance
                .instance_materials()
                .iter()
                .map(|material| {
                    (
                        material.symbol.clone(),
                        SemanticMappingTable::from(material),
                    )
                })
                .collect(),
        }
    }
}

impl From<&Instance<Controller>> for MeshInstance {
    fn from(instance: &Instance<Controller>) -> Self {
        let materials = instance
            .data
            .bind_material
            .as_ref()
            .map(|bind| {
                bind.instance_material
                    .iter()
                    .map(|material| {
                        (
                            material.symbol.clone(),
                            SemanticMappingTable::from(material),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        Self {
            mesh_or_controller: url_key(&instance.url.val).to_string(),
            materials,
        }
    }
}

/// Subset of a mesh with a certain material symbol.
#[derive(Clone, Debug, Default)]
struct SubMesh {
    material: String,
    num_faces: usize,
}

/// Intermediate Collada mesh data, assembled like Assimp's `Collada::Mesh`.
#[derive(Clone, Debug)]
pub(crate) struct Mesh {
    id: String,
    name: String,
    /// ID of the `<vertices>` element (for unsupported addressing checks).
    #[allow(dead_code)]
    vertex_id: String,
    #[allow(dead_code)]
    per_vertex_data: Vec<InputChannel>,
    /// Assembled vertex attributes in verbose (non-indexed) form.
    positions: Vec<AiVector3D>,
    #[allow(dead_code)]
    normals: Vec<AiVector3D>,
    #[allow(dead_code)]
    tangents: Vec<AiVector3D>,
    #[allow(dead_code)]
    bitangents: Vec<AiVector3D>,
    #[allow(dead_code)]
    tex_coords: [Vec<AiVector3D>; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    #[allow(dead_code)]
    colors: [Vec<AiColor4D>; AI_MAX_NUMBER_OF_COLORS_SETS],
    #[allow(dead_code)]
    num_uv_components: [u32; AI_MAX_NUMBER_OF_TEXTURECOORDS],
    /// Vertices per face: 1 == point, 2 == line, 3 == triangle, 4+ == poly.
    face_size: Vec<usize>,
    /// Position indices for all face corners (for bone weight assignment).
    #[allow(dead_code)]
    face_pos_indices: Vec<usize>,
    sub_meshes: Vec<SubMesh>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            vertex_id: String::new(),
            per_vertex_data: Vec::new(),
            positions: Vec::new(),
            normals: Vec::new(),
            tangents: Vec::new(),
            bitangents: Vec::new(),
            tex_coords: std::array::from_fn(|_| Vec::new()),
            colors: std::array::from_fn(|_| Vec::new()),
            num_uv_components: [2; AI_MAX_NUMBER_OF_TEXTURECOORDS],
            face_size: Vec::new(),
            face_pos_indices: Vec::new(),
            sub_meshes: Vec::new(),
        }
    }
}

/// Which type of primitives `Mesh::read_primitives` is going to read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PrimitiveType {
    Lines,
    LineStrip,
    Polygon,
    Polylist,
    Triangles,
    TriFans,
    TriStrips,
}

impl From<&dae_parser::InputS> for InputChannel {
    fn from(input: &dae_parser::InputS) -> Self {
        let type_ = InputType::from_semantic(&input.semantic);
        Self {
            type_,
            index: if matches!(type_, InputType::Texcoord | InputType::Color) {
                input.set.unwrap_or(0) as usize
            } else {
                0
            },
            offset: input.offset as usize,
            accessor: url_key(&input.source).to_string(),
        }
    }
}

impl Mesh {
    /// Reads input declarations of per-index mesh data into this mesh.
    /// Near-equivalent to Assimp's `ColladaParser::ReadIndexData`.
    fn read_index_data(
        &mut self,
        primitive: &Primitive,
        document_mesh: &DocumentMesh,
    ) -> Result<(), DaeImportError> {
        let (material, num_primitives, inputs, prim, vcount, prim_type) = match primitive {
            Primitive::Triangles(triangles) => (
                triangles.material.clone(),
                triangles.count,
                &triangles.inputs,
                triangles.data.prim.as_deref().unwrap_or(&[]),
                Vec::new(),
                PrimitiveType::Triangles,
            ),
            Primitive::PolyList(polylist) => (
                polylist.material.clone(),
                polylist.count,
                &polylist.inputs,
                polylist.data.prim.as_ref(),
                polylist
                    .data
                    .vcount
                    .iter()
                    .map(|&count| count as usize)
                    .collect(),
                PrimitiveType::Polylist,
            ),
            Primitive::Lines(lines) => (
                lines.material.clone(),
                lines.count,
                &lines.inputs,
                lines.data.prim.as_deref().unwrap_or(&[]),
                Vec::new(),
                PrimitiveType::Lines,
            ),
            Primitive::LineStrips(strips) => (
                strips.material.clone(),
                strips.count,
                &strips.inputs,
                &[][..],
                Vec::new(),
                PrimitiveType::LineStrip,
            ),
            Primitive::Polygons(polygons) => (
                polygons.material.clone(),
                polygons.count,
                &polygons.inputs,
                &[][..],
                Vec::new(),
                PrimitiveType::Polygon,
            ),
            Primitive::TriFans(fans) => (
                fans.material.clone(),
                fans.count,
                &fans.inputs,
                &[][..],
                Vec::new(),
                PrimitiveType::TriFans,
            ),
            Primitive::TriStrips(strips) => (
                strips.material.clone(),
                strips.count,
                &strips.inputs,
                &[][..],
                Vec::new(),
                PrimitiveType::TriStrips,
            ),
        };

        let mut subgroup = SubMesh {
            material: material.unwrap_or_default(),
            num_faces: 0,
        };

        let per_index_data: Vec<InputChannel> = inputs
            .iter()
            .map(InputChannel::from)
            .filter(|channel| channel.type_ != InputType::Invalid)
            .collect();

        // Assimp only assembles when a `<p>` is present; skip empty primitives.
        let actual_primitives = if prim.is_empty()
            && !matches!(
                prim_type,
                PrimitiveType::Polygon
                    | PrimitiveType::TriFans
                    | PrimitiveType::TriStrips
                    | PrimitiveType::LineStrip
            ) {
            0
        } else {
            self.read_primitives(
                document_mesh,
                &per_index_data,
                num_primitives,
                &vcount,
                prim,
                prim_type,
            )?
        };

        subgroup.num_faces = actual_primitives;
        self.sub_meshes.push(subgroup);
        Ok(())
    }

    /// Reads a primitive index list and assembles mesh data.
    /// Near-equivalent to Assimp's `ColladaParser::ReadPrimitives` (positions only for now).
    fn read_primitives(
        &mut self,
        document_mesh: &DocumentMesh,
        per_index_channels: &[InputChannel],
        num_primitives: usize,
        vcount: &[usize],
        indices: &[u32],
        prim_type: PrimitiveType,
    ) -> Result<usize, DaeImportError> {
        let num_offsets = per_index_channels
            .iter()
            .map(|channel| channel.offset + 1)
            .max()
            .unwrap_or(1);
        let vertex_channel = per_index_channels
            .iter()
            .find(|channel| channel.type_ == InputType::Vertex)
            .ok_or_else(|| DaeImportError::MissingLocalMapEntry("VERTEX".to_string()))?;
        let per_vertex_offset = vertex_channel.offset;
        if !self.vertex_id.is_empty() && vertex_channel.accessor != self.vertex_id {
            return Err(DaeImportError::MissingLocalMapEntry(
                vertex_channel.accessor.clone(),
            ));
        }

        let positions = position_reader(document_mesh, &vertex_channel.accessor)?;
        let position_count = positions.len();
        if per_vertex_offset >= num_offsets {
            return Err(DaeImportError::InvalidMeshIndices(format!(
                "VERTEX offset {per_vertex_offset} exceeds input stride {num_offsets}"
            )));
        }

        let mut actual_primitives = 0usize;
        match prim_type {
            PrimitiveType::Triangles => {
                let required = num_primitives
                    .checked_mul(3 * num_offsets)
                    .and_then(|corners| corners.checked_mul(num_offsets))
                    .ok_or_else(|| {
                        DaeImportError::InvalidMeshIndices(
                            "triangle index count overflow".to_string(),
                        )
                    })?;
                if indices.len() < required {
                    return Err(DaeImportError::InvalidMeshIndices(format!(
                        "triangles need at least {required} indices, found {}",
                        indices.len()
                    )));
                }
                for face in 0..num_primitives {
                    for corner in 0..3 {
                        let slot = (face * 3 + corner) * num_offsets + per_vertex_offset;
                        let index = indices[slot] as usize;
                        if index >= position_count {
                            return Err(DaeImportError::InvalidMeshIndices(format!(
                                "position index {index} out of bounds for {position_count} positions"
                            )));
                        }
                        self.positions.push(AiVector3D::from(
                            positions.get(index).map(|value| value as AiReal),
                        ));
                        self.face_pos_indices.push(index);
                    }
                    self.face_size.push(3);
                    actual_primitives += 1;
                }
            }
            PrimitiveType::Polylist => {
                let vertex_total: usize = vcount.iter().take(num_primitives).sum();
                let required = vertex_total.checked_mul(num_offsets).ok_or_else(|| {
                    DaeImportError::InvalidMeshIndices("polylist index count overflow".to_string())
                })?;
                if indices.len() < required {
                    return Err(DaeImportError::InvalidMeshIndices(format!(
                        "polylist needs at least {required} indices, found {}",
                        indices.len()
                    )));
                }
                let mut prim_cursor = 0usize;
                for &vertex_count in vcount.iter().take(num_primitives) {
                    for _ in 0..vertex_count {
                        let slot = prim_cursor * num_offsets + per_vertex_offset;
                        let index = indices[slot] as usize;
                        prim_cursor += 1;
                        if index >= position_count {
                            return Err(DaeImportError::InvalidMeshIndices(format!(
                                "position index {index} out of bounds for {position_count} positions"
                            )));
                        }
                        self.positions.push(AiVector3D::from(
                            positions.get(index).map(|value| value as AiReal),
                        ));
                        self.face_pos_indices.push(index);
                    }
                    self.face_size.push(vertex_count);
                    actual_primitives += 1;
                }
            }
            // Other primitive kinds are recognized but not assembled yet.
            PrimitiveType::Lines
            | PrimitiveType::LineStrip
            | PrimitiveType::Polygon
            | PrimitiveType::TriFans
            | PrimitiveType::TriStrips => {}
        }

        Ok(actual_primitives)
    }
}

impl TryFrom<&Geometry> for Mesh {
    type Error = DaeImportError;

    fn try_from(geometry: &Geometry) -> Result<Self, Self::Error> {
        let document_mesh = geometry
            .element
            .as_mesh()
            .ok_or_else(|| DaeImportError::MissingLocalMapEntry("mesh".to_string()))?;
        let mut mesh = Mesh {
            id: geometry.id.clone().unwrap_or_default(),
            name: geometry.name.clone().unwrap_or_default(),
            ..Default::default()
        };

        if let Some(vertices) = &document_mesh.vertices {
            mesh.vertex_id = vertices.id.clone();
            mesh.per_vertex_data = vertices
                .inputs
                .iter()
                .map(|input| InputChannel {
                    type_: InputType::from_semantic(&input.semantic),
                    index: 0,
                    offset: 0,
                    accessor: url_key(&input.source).to_string(),
                })
                .collect();
        }

        for primitive in &document_mesh.elements {
            mesh.read_index_data(primitive, document_mesh)?;
        }

        Ok(mesh)
    }
}

impl DaeImporter {
    pub(crate) fn import_mesh_library(
        &self,
        document: &Document,
    ) -> Result<HashMap<String, Mesh>, DaeImportError> {
        let mut library = HashMap::new();
        for geometry in document.iter::<Geometry>() {
            let Some(id) = geometry.id.as_ref().filter(|id| !id.is_empty()) else {
                continue;
            };
            if library.contains_key(id) {
                continue;
            }
            if let Ok(mesh) = Mesh::try_from(geometry) {
                library.insert(id.clone(), mesh);
            }
        }
        Ok(library)
    }

    pub(crate) fn build_meshes_for_node(
        &self,
        document: &Document,
        node: &Node,
        mesh_library: &HashMap<String, Mesh>,
        material_index_map: &HashMap<String, usize>,
    ) -> Result<(Vec<AiMesh>, HashMap<String, usize>), DaeImportError> {
        let controller_map = document
            .local_map::<Controller>()
            .map_err(DaeImportError::FileFormatError)?;
        let material_map = document
            .local_map::<Material>()
            .map_err(DaeImportError::FileFormatError)?;

        let mut meshes = Vec::new();
        let mut name_index_map = HashMap::new();

        let instances = node
            .instance_geometry
            .iter()
            .map(MeshInstance::from)
            .chain(node.instance_controller.iter().map(MeshInstance::from));

        for instance in instances {
            let Some(src_mesh) =
                resolve_mesh(&instance.mesh_or_controller, mesh_library, &controller_map)
            else {
                continue;
            };

            let first_index = meshes.len();
            let mut added = false;
            let mut vertex_start = 0usize;
            let mut face_start = 0usize;

            for sub_mesh in &src_mesh.sub_meshes {
                if sub_mesh.num_faces == 0 {
                    continue;
                }

                let table = instance.material_for_symbol(&sub_mesh.material);
                let mat_name = table.map(|table| table.mat_name.as_str()).unwrap_or("");
                let material_index =
                    resolve_material_index(document, mat_name, &material_map, material_index_map);

                let num_vertices = src_mesh.face_size[face_start..face_start + sub_mesh.num_faces]
                    .iter()
                    .sum::<usize>();
                let mut faces = Vec::with_capacity(sub_mesh.num_faces);
                let mut primitive_types = BitFlags::empty();
                let mut vertex = 0usize;
                for &face_size in &src_mesh.face_size[face_start..face_start + sub_mesh.num_faces] {
                    match face_size {
                        1 => primitive_types |= AiPrimitiveType::Point,
                        2 => primitive_types |= AiPrimitiveType::Line,
                        3 => primitive_types |= AiPrimitiveType::Triangle,
                        _ => primitive_types |= AiPrimitiveType::Polygon,
                    }
                    let mut indices = Vec::with_capacity(face_size);
                    for _ in 0..face_size {
                        indices.push(vertex);
                        vertex += 1;
                    }
                    faces.push(indices);
                }

                let name = if self.use_collada_name && !src_mesh.name.is_empty() {
                    src_mesh.name.clone()
                } else {
                    src_mesh.id.clone()
                };

                meshes.push(AiMesh {
                    name,
                    primitive_types,
                    vertices: src_mesh.positions[vertex_start..vertex_start + num_vertices]
                        .to_vec(),
                    faces,
                    material_index,
                    ..AiMesh::default()
                });
                added = true;
                vertex_start += num_vertices;
                face_start += sub_mesh.num_faces;
            }

            if added {
                name_index_map.insert(instance.mesh_or_controller, first_index);
            }
        }

        Ok((meshes, name_index_map))
    }
}

fn resolve_mesh<'a>(
    mesh_or_controller: &str,
    mesh_library: &'a HashMap<String, Mesh>,
    controller_map: &LocalMap<'_, Controller>,
) -> Option<&'a Mesh> {
    if let Some(mesh) = mesh_library.get(mesh_or_controller) {
        return Some(mesh);
    }
    let controller = controller_map.get_str(mesh_or_controller)?;
    let mesh_id = url_key(controller.element.source());
    mesh_library.get(mesh_id)
}

fn resolve_material_index(
    document: &Document,
    mat_name: &str,
    material_map: &LocalMap<'_, Material>,
    material_index_map: &HashMap<String, usize>,
) -> u32 {
    if mat_name.is_empty() {
        return 0;
    }
    let Some(material) = material_map.get_str(mat_name) else {
        return 0;
    };
    let Some(item_index) = document.library_iter::<Material>().find_map(|library| {
        library
            .items
            .iter()
            .position(|item| std::ptr::eq(item, material))
    }) else {
        return 0;
    };
    material_index_map
        .get(&material_key(material, item_index))
        .copied()
        .unwrap_or(0) as u32
}

fn position_reader<'a>(
    document_mesh: &'a DocumentMesh,
    vertices_id: &str,
) -> Result<SourceReader<'a, XYZ>, DaeImportError> {
    let vertices = document_mesh
        .vertices
        .as_ref()
        .filter(|vertices| vertices.id == vertices_id)
        .ok_or_else(|| DaeImportError::MissingLocalMapEntry(vertices_id.to_string()))?;
    let position = vertices.position_input();
    let source = find_source(&document_mesh.sources, &position.source)
        .ok_or_else(|| DaeImportError::MissingLocalMapEntry(position.source.to_string()))?;
    source
        .reader(XYZ)
        .ok_or_else(|| DaeImportError::MissingLocalMapEntry(position.source.to_string()))
}

fn find_source<'a>(sources: &'a [Source], url: &Url) -> Option<&'a Source> {
    let id = url_key(url);
    sources
        .iter()
        .find(|source| source.id.as_deref() == Some(id))
}

fn url_key(url: &Url) -> &str {
    match url {
        Url::Fragment(fragment) => fragment,
        Url::Other(other) => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dae_parser::Document;
    use std::str::FromStr;

    const CUBE_DAE: &str = include_str!("../../tests/cube.dae");

    fn cube_document() -> Document {
        Document::from_str(CUBE_DAE).expect("cube.dae should parse")
    }

    fn cube_geometry(document: &Document) -> &Geometry {
        document
            .iter::<Geometry>()
            .find(|geometry| geometry.id.as_deref() == Some("F1"))
            .expect("cube geometry F1")
    }

    #[test]
    fn try_from_geometry_sets_identity_and_vertices() {
        let document = cube_document();
        let mesh = Mesh::try_from(cube_geometry(&document)).expect("mesh from geometry");

        assert_eq!(mesh.id, "F1");
        assert_eq!(mesh.name, "Face1Geometry");
        assert_eq!(mesh.vertex_id, "cube-vertices");
        assert_eq!(mesh.per_vertex_data.len(), 1);
        assert_eq!(mesh.per_vertex_data[0].type_, InputType::Position);
        assert_eq!(mesh.per_vertex_data[0].accessor, "cube-vertex-positions");
        assert_eq!(mesh.per_vertex_data[0].offset, 0);
        assert_eq!(mesh.per_vertex_data[0].index, 0);
    }

    #[test]
    fn try_from_geometry_assembles_triangle_submesh() {
        let document = cube_document();
        let mesh = Mesh::try_from(cube_geometry(&document)).expect("mesh from geometry");

        assert_eq!(mesh.sub_meshes.len(), 1);
        assert_eq!(mesh.sub_meshes[0].material, "geometryElement5");
        assert_eq!(mesh.sub_meshes[0].num_faces, 12);
        assert_eq!(mesh.face_size, vec![3; 12]);
        assert_eq!(mesh.positions.len(), 36);
        assert_eq!(mesh.face_pos_indices.len(), 36);
        assert_eq!(
            mesh.face_pos_indices,
            vec![
                0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15,
                16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
            ]
        );
    }

    #[test]
    fn try_from_geometry_resolves_verbose_positions() {
        let document = cube_document();
        let mesh = Mesh::try_from(cube_geometry(&document)).expect("mesh from geometry");

        assert_eq!(mesh.positions[0], AiVector3D::new(-50.0, 50.0, 50.0));
        assert_eq!(mesh.positions[1], AiVector3D::new(-50.0, -50.0, 50.0));
        assert_eq!(mesh.positions[2], AiVector3D::new(50.0, -50.0, 50.0));
        // First corner of the second triangle reuses index 0.
        assert_eq!(mesh.positions[3], mesh.positions[0]);
        assert_eq!(mesh.positions[4], mesh.positions[2]);
        assert_eq!(mesh.positions[5], AiVector3D::new(50.0, 50.0, 50.0));
    }

    #[test]
    fn mesh_instance_binds_material_symbol() {
        let document = cube_document();
        let node = document
            .get_visual_scene()
            .expect("visual scene")
            .nodes
            .first()
            .expect("root node");
        let instance = MeshInstance::from(&node.instance_geometry[0]);

        assert_eq!(instance.mesh_or_controller, "F1");
        assert_eq!(instance.materials.len(), 1);
        let table = instance
            .material_for_symbol("geometryElement5")
            .expect("bound subgroup");
        assert_eq!(table.mat_name, "Blue");
        assert!(table.map.is_empty());
    }

    #[test]
    fn import_mesh_library_indexes_cube_geometry() {
        let document = cube_document();
        let library = DaeImporter::new()
            .import_mesh_library(&document)
            .expect("mesh library");

        assert_eq!(library.len(), 1);
        let mesh = library.get("F1").expect("F1 mesh");
        assert_eq!(mesh.name, "Face1Geometry");
        assert_eq!(mesh.sub_meshes[0].num_faces, 12);
    }

    #[test]
    fn build_meshes_for_node_emits_ai_mesh() {
        let document = cube_document();
        let importer = DaeImporter::new();
        let library = importer
            .import_mesh_library(&document)
            .expect("mesh library");
        let (materials, material_index_map) =
            importer.import_materials(&document).expect("materials");
        assert_eq!(materials.len(), 1);

        let node = document
            .get_visual_scene()
            .expect("visual scene")
            .nodes
            .first()
            .expect("root node");
        let (meshes, name_map) = importer
            .build_meshes_for_node(&document, node, &library, &material_index_map)
            .expect("node meshes");

        assert_eq!(meshes.len(), 1);
        assert_eq!(name_map.get("F1"), Some(&0));
        let mesh = &meshes[0];
        assert_eq!(mesh.name, "F1");
        assert_eq!(mesh.vertices.len(), 36);
        assert_eq!(mesh.faces.len(), 12);
        assert_eq!(mesh.faces[0], vec![0, 1, 2]);
        assert_eq!(mesh.faces[1], vec![3, 4, 5]);
        assert_eq!(mesh.material_index, 0);
        assert!(mesh.primitive_types.contains(AiPrimitiveType::Triangle));
        assert_eq!(mesh.vertices[0], AiVector3D::new(-50.0, 50.0, 50.0));
    }
}
