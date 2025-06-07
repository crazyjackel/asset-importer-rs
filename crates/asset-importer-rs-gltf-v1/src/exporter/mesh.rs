use std::collections::HashMap;

use asset_importer_rs_scene::{
    AI_MAX_NUMBER_OF_TEXTURECOORDS, AiPrimitiveType, AiQuaternion, AiScene, AiVector2D, AiVector3D,
};
use gltf_v1::json::{
    Accessor, BufferView, Mesh, Root, StringIndex,
    accessor::{ComponentType, Type},
    buffer::BufferViewType,
    mesh::{Primitive, PrimitiveMode, Semantic},
    validation::{Checked, USize64},
};

use crate::{
    GltfExporter,
    exporter::{error::Error, generate_unique_name},
};

impl GltfExporter {
    pub(crate) fn export_meshes(
        &self,
        scene: &AiScene,
        root: &mut Root,
        body_buffer_data: &mut Vec<u8>,
        mesh_index_map: &HashMap<usize, String>,
        material_index_map: &HashMap<usize, String>,
    ) -> Result<(), Error> {
        //@TODO: Add support for OPEN3DGC
        //@TODO: Add support for skins
        // let mut create_skin = false;
        // for mesh in &scene.meshes {
        //     if !mesh.bones.is_empty() {
        //         create_skin = true;
        //     }
        // }
        let mut unique_names_map: HashMap<String, u32> = HashMap::new();

        for mesh_index in 0..scene.meshes.len() {
            let ai_mesh = &scene.meshes[mesh_index];
            let mesh_name = mesh_index_map.get(&mesh_index).unwrap();
            let mut mesh = Mesh {
                primitives: Vec::with_capacity(1),
                name: Some(mesh_name.clone()),
            };

            let material_name = material_index_map
                .get(&(ai_mesh.material_index as usize))
                .ok_or(Error::MissingMaterial)?;
            let mut primitive = Primitive::new(StringIndex::new(material_name.clone()));

            // Positions
            let positions = export_vector_3d(
                root,
                body_buffer_data,
                &ai_mesh.vertices,
                &mut unique_names_map,
            );
            primitive
                .attributes
                .insert(Checked::Valid(Semantic::Positions), positions);

            // Normals
            let normals = export_vector_3d(
                root,
                body_buffer_data,
                &ai_mesh.normals,
                &mut unique_names_map,
            );
            primitive
                .attributes
                .insert(Checked::Valid(Semantic::Normals), normals);

            // Texture Coordinates
            for i in 0..AI_MAX_NUMBER_OF_TEXTURECOORDS {
                let uv = &ai_mesh.texture_coords[i];
                if uv.is_none() {
                    continue;
                }
                let ai_uvs = uv.as_ref().unwrap();
                let is_2d = ai_uvs.iter().all(|x| x.z == 0.0);
                let uvs = if is_2d {
                    export_vector_2d(
                        root,
                        body_buffer_data,
                        &ai_uvs
                            .iter()
                            .map(|x| AiVector2D::new(x.x, 1.0 - x.y))
                            .collect(),
                        &mut unique_names_map,
                    )
                } else {
                    export_vector_3d(
                        root,
                        body_buffer_data,
                        &ai_uvs
                            .iter()
                            .map(|x| AiVector3D::new(x.x, 1.0 - x.y, x.z))
                            .collect(),
                        &mut unique_names_map,
                    )
                };
                primitive
                    .attributes
                    .insert(Checked::Valid(Semantic::TexCoords(i as u32)), uvs);
            }

            // Indices
            if !ai_mesh.faces.is_empty() {
                let indices_per_face = ai_mesh.faces[0].len();
                let mut indices = Vec::with_capacity(ai_mesh.faces.len() * indices_per_face);
                for i in 0..ai_mesh.faces.len() {
                    for j in 0..indices_per_face {
                        indices.push(ai_mesh.faces[i][j] as u16);
                    }
                }
                let indices = export_short(root, body_buffer_data, &indices, &mut unique_names_map);
                primitive.indices = Some(indices);
            }

            primitive.mode = if ai_mesh.primitive_types.contains(AiPrimitiveType::Triangle) {
                Checked::Valid(PrimitiveMode::Triangles)
            } else if ai_mesh.primitive_types.contains(AiPrimitiveType::Line) {
                Checked::Valid(PrimitiveMode::Lines)
            } else if ai_mesh.primitive_types.contains(AiPrimitiveType::Point) {
                Checked::Valid(PrimitiveMode::Points)
            } else {
                Checked::Valid(PrimitiveMode::Triangles)
            };

            mesh.primitives.push(primitive);

            root.meshes.insert(mesh_name.clone(), mesh);
        }
        Ok(())
    }
}

pub struct AccessorExporter {
    target: BufferViewType,        //ARRAY_BUFFER or ELEMENT_ARRAY_BUFFER
    type_in: Type,                 //VEC3, VEC4, etc.
    type_out: Type,                //VEC3, VEC4, etc.
    component_type: ComponentType, //FLOAT, UNSIGNED_SHORT, etc.
    count: u32,                    //number of elements of component_type grouped by type_in
    min: Vec<f32>,                 //length = type_out.get_num_components()
    max: Vec<f32>,                 //length = type_out.get_num_components()
}

pub(crate) fn export_short(
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    short_data: &[u16],
    unique_names_map: &mut HashMap<String, u32>,
) -> StringIndex<Accessor> {
    let mut data: Vec<u8> = Vec::with_capacity(short_data.len() * 2);
    let mut min: f32 = f32::MAX;
    let mut max: f32 = f32::MIN;
    for value in short_data {
        let value_f = *value as f32;
        if value_f < min {
            min = value_f;
        }
        if value_f > max {
            max = value_f;
        }
        data.extend_from_slice(&value.to_le_bytes());
    }
    export_data(
        root,
        buffer_data,
        &data,
        unique_names_map,
        AccessorExporter {
            target: BufferViewType::ElementArrayBuffer,
            type_in: Type::SCALAR,
            type_out: Type::SCALAR,
            component_type: ComponentType::UnsignedShort,
            count: short_data.len() as u32,
            min: vec![min],
            max: vec![max],
        },
    )
}

pub(crate) fn export_float(
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    float_data: &[f32],
    unique_names_map: &mut HashMap<String, u32>,
) -> StringIndex<Accessor> {
    let mut data: Vec<u8> = Vec::with_capacity(float_data.len() * 4);
    let mut min: f32 = f32::MAX;
    let mut max: f32 = f32::MIN;
    for value in float_data {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
        data.extend_from_slice(&value.to_le_bytes());
    }
    export_data(
        root,
        buffer_data,
        &data,
        unique_names_map,
        AccessorExporter {
            target: BufferViewType::ElementArrayBuffer,
            type_in: Type::SCALAR,
            type_out: Type::SCALAR,
            component_type: ComponentType::Float,
            count: float_data.len() as u32,
            min: vec![min],
            max: vec![max],
        },
    )
}

pub(crate) fn export_vector_2d(
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    vector_data: &Vec<AiVector2D>,
    unique_names_map: &mut HashMap<String, u32>,
) -> StringIndex<Accessor> {
    let mut min: [f32; 2] = if vector_data.is_empty() {
        [0.0; 2]
    } else {
        [f32::MAX; 2]
    };
    let mut max: [f32; 2] = if vector_data.is_empty() {
        [0.0; 2]
    } else {
        [f32::MIN; 2]
    };
    let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 2 * 4);
    for vector in vector_data {
        for i in 0..2_usize {
            if vector[i] < min[i] {
                min[i] = vector[i];
            }
            if vector[i] > max[i] {
                max[i] = vector[i];
            }
            data.extend_from_slice(&vector[i].to_le_bytes());
        }
    }
    export_data(
        root,
        buffer_data,
        &data,
        unique_names_map,
        AccessorExporter {
            target: BufferViewType::ArrayBuffer,
            type_in: Type::VEC2,
            type_out: Type::VEC2,
            component_type: ComponentType::Float,
            count: vector_data.len() as u32,
            min: min.to_vec(),
            max: max.to_vec(),
        },
    )
}

pub(crate) fn export_vector_3d(
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    vector_data: &Vec<AiVector3D>,
    unique_names_map: &mut HashMap<String, u32>,
) -> StringIndex<Accessor> {
    let mut min: [f32; 3] = if vector_data.is_empty() {
        [0.0; 3]
    } else {
        [f32::MAX; 3]
    };
    let mut max: [f32; 3] = if vector_data.is_empty() {
        [0.0; 3]
    } else {
        [f32::MIN; 3]
    };
    let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 3 * 4);
    for vector in vector_data {
        for i in 0..3_usize {
            if vector[i] < min[i] {
                min[i] = vector[i];
            }
            if vector[i] > max[i] {
                max[i] = vector[i];
            }
            data.extend_from_slice(&vector[i].to_le_bytes());
        }
    }
    export_data(
        root,
        buffer_data,
        &data,
        unique_names_map,
        AccessorExporter {
            target: BufferViewType::ArrayBuffer,
            type_in: Type::VEC3,
            type_out: Type::VEC3,
            component_type: ComponentType::Float,
            count: vector_data.len() as u32,
            min: min.to_vec(),
            max: max.to_vec(),
        },
    )
}

pub(crate) fn export_vector_4d(
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    vector_data: &Vec<AiQuaternion>,
    unique_names_map: &mut HashMap<String, u32>,
) -> StringIndex<Accessor> {
    let mut min: [f32; 4] = if vector_data.is_empty() {
        [0.0; 4]
    } else {
        [f32::MAX; 4]
    };
    let mut max: [f32; 4] = if vector_data.is_empty() {
        [0.0; 4]
    } else {
        [f32::MIN; 4]
    };
    let mut data: Vec<u8> = Vec::with_capacity(vector_data.len() * 4 * 4);
    for vector in vector_data {
        for i in 0..4_usize {
            if vector[i] < min[i] {
                min[i] = vector[i];
            }
            if vector[i] > max[i] {
                max[i] = vector[i];
            }
            data.extend_from_slice(&vector[i].to_le_bytes());
        }
    }
    export_data(
        root,
        buffer_data,
        &data,
        unique_names_map,
        AccessorExporter {
            target: BufferViewType::ArrayBuffer,
            type_in: Type::VEC4,
            type_out: Type::VEC4,
            component_type: ComponentType::Float,
            count: vector_data.len() as u32,
            min: min.to_vec(),
            max: max.to_vec(),
        },
    )
}

pub(crate) fn export_data(
    root: &mut Root,
    buffer_data: &mut Vec<u8>,
    data: &[u8],
    unique_names_map: &mut HashMap<String, u32>,
    exporter: AccessorExporter,
) -> StringIndex<Accessor> {
    let num_components_in = exporter.type_in.get_num_components() as usize;
    let num_components_out = exporter.type_out.get_num_components() as usize;
    let bytes_per_component = exporter.component_type.size() as usize;

    //align to byte boundary
    let mut offset = buffer_data.len();
    let padding = offset % bytes_per_component;
    offset += padding;
    let length = exporter.count as usize * num_components_out * bytes_per_component;
    buffer_data.reserve(offset + length);
    for _ in 0..padding {
        buffer_data.push(0);
    }

    assert!(data.len() == exporter.count as usize * num_components_in * bytes_per_component);
    if num_components_in == num_components_out {
        for i in 0..exporter.count as usize {
            for j in 0..num_components_out {
                buffer_data.push(data[i * num_components_out + j]);
            }
        }
    } else {
        let smaller_size = num_components_in.min(num_components_out);
        for i in 0..exporter.count as usize {
            for j in 0..smaller_size {
                buffer_data.push(data[i * num_components_out + j]);
            }
            for _ in smaller_size..num_components_out {
                buffer_data.push(0);
            }
        }
    }

    let view_name = generate_unique_name("view", unique_names_map);
    let view = BufferView {
        byte_offset: USize64(offset as u64),
        byte_length: USize64(length as u64),
        target: Some(Checked::Valid(exporter.target)),
        name: Some(view_name.clone()),
        buffer: StringIndex::new("body".to_string()),
    };
    root.buffer_views.insert(view_name.clone(), view);

    let accessor_name = generate_unique_name("accessor", unique_names_map);
    let accessor = Accessor {
        buffer_view: StringIndex::new(view_name),
        byte_offset: 0,
        byte_stride: Some(0),
        component_type: Checked::Valid(exporter.component_type),
        count: exporter.count,
        type_: Checked::Valid(exporter.type_out),
        max: exporter.max,
        min: exporter.min,
        name: Some(accessor_name.clone()),
    };
    root.accessors.insert(accessor_name.clone(), accessor);
    StringIndex::new(accessor_name)
}

#[cfg(test)]
mod tests {
    use crate::Output;

    use super::*;
    use asset_importer_rs_scene::{AiMesh, AiPrimitiveType, AiVector2D, AiVector3D};
    use std::collections::HashMap;

    fn create_test_mesh() -> AiMesh {
        let mut mesh = AiMesh {
            name: "mesh_0".to_string(),
            primitive_types: AiPrimitiveType::Triangle.into(),
            vertices: vec![
                AiVector3D::new(0.0, 0.0, 0.0),
                AiVector3D::new(1.0, 0.0, 0.0),
                AiVector3D::new(0.0, 1.0, 0.0),
            ],
            normals: vec![
                AiVector3D::new(0.0, 0.0, 1.0),
                AiVector3D::new(0.0, 0.0, 1.0),
                AiVector3D::new(0.0, 0.0, 1.0),
            ],
            faces: vec![vec![0, 1, 2]],
            ..Default::default()
        };

        // Add texture coordinates
        mesh.texture_coords[0] = Some(vec![
            AiVector3D::new(0.0, 0.0, 0.0),
            AiVector3D::new(1.0, 0.0, 0.0),
            AiVector3D::new(0.0, 1.0, 0.0),
        ]);

        mesh
    }

    #[test]
    fn test_export_meshes_basic() {
        let mut scene = AiScene::default();
        scene.meshes.push(create_test_mesh());

        let mut root = Root::default();
        let mut body_buffer_data = Vec::new();
        let mut mesh_index_map = HashMap::new();
        mesh_index_map.insert(0, "mesh_0".to_string());
        let mut material_index_map = HashMap::new();
        material_index_map.insert(0, "material_0".to_string());

        let exporter = GltfExporter::new(Output::Standard);

        let result = exporter.export_meshes(
            &scene,
            &mut root,
            &mut body_buffer_data,
            &mesh_index_map,
            &material_index_map,
        );
        assert!(result.is_ok());

        // Check if mesh was exported
        assert!(!root.meshes.is_empty());

        // Get the exported mesh
        let mesh = root.meshes.get("mesh_0").unwrap();

        // Check mesh name
        assert_eq!(mesh.name.as_ref().unwrap(), "mesh_0");

        // Check primitive
        assert_eq!(mesh.primitives.len(), 1);
        let primitive = &mesh.primitives[0];

        // Check material reference
        assert_eq!(primitive.material.value(), "material_0");

        // Check primitive mode
        assert_eq!(primitive.mode, Checked::Valid(PrimitiveMode::Triangles));

        // Check attributes
        assert!(
            primitive
                .attributes
                .contains_key(&Checked::Valid(Semantic::Positions))
        );
        assert!(
            primitive
                .attributes
                .contains_key(&Checked::Valid(Semantic::Normals))
        );
        assert!(
            primitive
                .attributes
                .contains_key(&Checked::Valid(Semantic::TexCoords(0)))
        );

        // Check indices
        assert!(primitive.indices.is_some());
    }

    #[test]
    fn test_export_meshes_without_texture_coords() {
        let mut scene = AiScene::default();
        let mut mesh = create_test_mesh();
        mesh.texture_coords = [const { None }; AI_MAX_NUMBER_OF_TEXTURECOORDS];
        scene.meshes.push(mesh);

        let mut root = Root::default();
        let mut body_buffer_data = Vec::new();
        let mut mesh_index_map = HashMap::new();
        mesh_index_map.insert(0, "mesh_0".to_string());
        let mut material_index_map = HashMap::new();
        material_index_map.insert(0, "material_0".to_string());

        let exporter = GltfExporter::new(Output::Standard);

        let result = exporter.export_meshes(
            &scene,
            &mut root,
            &mut body_buffer_data,
            &mesh_index_map,
            &material_index_map,
        );
        assert!(result.is_ok());

        let mesh = root.meshes.get("mesh_0").unwrap();
        let primitive = &mesh.primitives[0];

        // Check that texture coordinates are not present
        assert!(
            !primitive
                .attributes
                .contains_key(&Checked::Valid(Semantic::TexCoords(0)))
        );
    }

    #[test]
    fn test_export_data_vector2d() {
        let mut root = Root::default();
        let mut buffer_data = Vec::new();
        let mut unique_names_map = HashMap::new();

        let test_data = vec![
            AiVector2D::new(0.0, 0.0),
            AiVector2D::new(1.0, 1.0),
            AiVector2D::new(-1.0, -1.0),
        ];

        let result = export_vector_2d(
            &mut root,
            &mut buffer_data,
            &test_data,
            &mut unique_names_map,
        );

        // Check that buffer data was written
        assert!(!buffer_data.is_empty());

        // Check that buffer view was created
        assert!(!root.buffer_views.is_empty());

        // Check that accessor was created
        assert!(!root.accessors.is_empty());

        // Get the accessor
        let accessor = root.accessors.get(result.value()).unwrap();

        // Check accessor properties
        assert_eq!(accessor.count, 3);
        assert_eq!(
            accessor.component_type,
            Checked::Valid(ComponentType::Float)
        );
        assert_eq!(accessor.type_, Checked::Valid(Type::VEC2));

        // Check min/max values
        assert_eq!(accessor.min, vec![-1.0, -1.0]);
        assert_eq!(accessor.max, vec![1.0, 1.0]);
    }

    #[test]
    fn test_export_data_vector3d() {
        let mut root = Root::default();
        let mut buffer_data = Vec::new();
        let mut unique_names_map = HashMap::new();

        let test_data = vec![
            AiVector3D::new(0.0, 0.0, 0.0),
            AiVector3D::new(1.0, 1.0, 1.0),
            AiVector3D::new(-1.0, -1.0, -1.0),
        ];

        let result = export_vector_3d(
            &mut root,
            &mut buffer_data,
            &test_data,
            &mut unique_names_map,
        );

        // Check that buffer data was written
        assert!(!buffer_data.is_empty());

        // Check that buffer view was created
        assert!(!root.buffer_views.is_empty());

        // Check that accessor was created
        assert!(!root.accessors.is_empty());

        // Get the accessor
        let accessor = root.accessors.get(result.value()).unwrap();

        // Check accessor properties
        assert_eq!(accessor.count, 3);
        assert_eq!(
            accessor.component_type,
            Checked::Valid(ComponentType::Float)
        );
        assert_eq!(accessor.type_, Checked::Valid(Type::VEC3));

        // Check min/max values
        assert_eq!(accessor.min, vec![-1.0, -1.0, -1.0]);
        assert_eq!(accessor.max, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_export_data_short() {
        let mut root = Root::default();
        let mut buffer_data = Vec::new();
        let mut unique_names_map = HashMap::new();

        let test_data = vec![0u16, 1u16, 2u16, 3u16];

        let result = export_short(
            &mut root,
            &mut buffer_data,
            &test_data,
            &mut unique_names_map,
        );

        // Check that buffer data was written
        assert!(!buffer_data.is_empty());

        // Check that buffer view was created
        assert!(!root.buffer_views.is_empty());

        // Check that accessor was created
        assert!(!root.accessors.is_empty());

        // Get the accessor
        let accessor = root.accessors.get(result.value()).unwrap();

        // Check accessor properties
        assert_eq!(accessor.count, 4);
        assert_eq!(
            accessor.component_type,
            Checked::Valid(ComponentType::UnsignedShort)
        );
        assert_eq!(accessor.type_, Checked::Valid(Type::SCALAR));

        // Check min/max values
        assert_eq!(accessor.min, vec![0.0]);
        assert_eq!(accessor.max, vec![3.0]);
    }
}
