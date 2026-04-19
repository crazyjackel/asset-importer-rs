//! FBX `Geometry` / `Mesh` — Assimp [`MeshGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.h) / [`FBXMeshGeometry.cpp`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.cpp).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::OwnedObject;
use fbxscii::{ElementAmphitheatre, ElementAttribute};

use super::AttrExtractor;
use super::AttrExtractorExt;
use super::{FbxObjectTag, FbxTryFromReason, FbxTypeMismatch, fbx_object_tag};

const MAX_UV_CHANNELS: usize = 8;
const MAX_COLOR_SETS: usize = 8;

const ATTR_MAPPING_INFORMATION_TYPE: &str = "MappingInformationType";
const ATTR_REFERENCE_INFORMATION_TYPE: &str = "ReferenceInformationType";

/// FBX SDK spelling (not "ByVertex").
const MAPPING_BY_VERTICE: &str = "ByVertice";
const MAPPING_BY_POLYGON_VERTEX: &str = "ByPolygonVertex";
const MAPPING_BY_POLYGON: &str = "ByPolygon";
const MAPPING_ALL_SAME: &str = "AllSame";

const REFERENCE_DIRECT: &str = "Direct";
const REFERENCE_INDEX_TO_DIRECT: &str = "IndexToDirect";

#[derive(Debug, PartialEq)]
pub struct MeshGeometry {
    pub object: OwnedObject,
    pub vertices: Vec<[f32; 3]>,
    pub face_vertex_counts: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub tangents: Vec<[f32; 3]>,
    pub binormals: Vec<[f32; 3]>,
    pub texture_coords: [Vec<[f32; 2]>; MAX_UV_CHANNELS],
    pub texture_coord_names: [String; MAX_UV_CHANNELS],
    pub vertex_colors: [Vec<[f32; 4]>; MAX_COLOR_SETS],
    pub material_indices: Vec<i32>,
}

impl MeshGeometry {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

impl TryFrom<OwnedObject> for MeshGeometry {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::MeshGeometry) => {}
            _ => {
                return Err(FbxTypeMismatch::wrong_object_kind(
                    o,
                    "MeshGeometry".to_string(),
                ));
            }
        }

        let attrs = &o.attributes;

        let verts_attr = match attrs.extract_case_insensitive("Vertices") {
            Some(a) => a,
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: "Vertices".to_string(),
                    },
                ));
            }
        };
        let verts_flat = match parse_f32_array(verts_attr, "Vertices") {
            Ok(v) => v,
            Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
        };
        let temp_verts = match vec3_positions_from_flat(&verts_flat, "Vertices") {
            Ok(v) => v,
            Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
        };

        let poly_attr = match attrs.extract_case_insensitive("PolygonVertexIndex") {
            Some(a) => a,
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: "PolygonVertexIndex".to_string(),
                    },
                ));
            }
        };
        let temp_faces = match parse_i32_array(poly_attr, "PolygonVertexIndex") {
            Ok(v) => v,
            Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
        };

        let (vertices, face_vertex_counts, mapping_counts, mapping_offsets, mappings) =
            match expand_mesh_polygon_vertices(&temp_verts, &temp_faces) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        let vertex_count = vertices.len();

        let mut normals = Vec::new();
        if let Some(el) = attrs.extract_case_insensitive("LayerElementNormal") {
            let map = match child_attribute_map(el) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            normals = match resolve_vec3_channel(
                &map,
                "Normals",
                "NormalsIndex",
                vertex_count,
                &mapping_counts,
                &mapping_offsets,
                &mappings,
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        }

        let mut tangents = Vec::new();
        if let Some(el) = attrs.extract_case_insensitive("LayerElementTangent") {
            let map = match child_attribute_map(el) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let (data_name, index_name) = if map.extract_case_insensitive("Tangents").is_some() {
                ("Tangents", "TangentsIndex")
            } else {
                ("Tangent", "TangentIndex")
            };
            tangents = match resolve_vec3_channel(
                &map,
                data_name,
                index_name,
                vertex_count,
                &mapping_counts,
                &mapping_offsets,
                &mappings,
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        }

        let mut binormals = Vec::new();
        if let Some(el) = attrs.extract_case_insensitive("LayerElementBinormal") {
            let map = match child_attribute_map(el) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let (data_name, index_name) = if map.extract_case_insensitive("Binormals").is_some() {
                ("Binormals", "BinormalsIndex")
            } else {
                ("Binormal", "BinormalIndex")
            };
            binormals = match resolve_vec3_channel(
                &map,
                data_name,
                index_name,
                vertex_count,
                &mapping_counts,
                &mapping_offsets,
                &mappings,
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        }

        let mut texture_coords: [Vec<[f32; 2]>; MAX_UV_CHANNELS] =
            std::array::from_fn(|_| Vec::new());
        let mut texture_coord_names: [String; MAX_UV_CHANNELS] =
            std::array::from_fn(|_| String::new());

        if let Some(el) = attrs.extract_case_insensitive("LayerElementUV") {
            let map = match child_attribute_map(el) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            if let Ok(Some(name)) = map.optional_token_case_insensitive("Name") {
                texture_coord_names[0] = name
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'')
                    .to_string();
            }
            texture_coords[0] = match resolve_vec2_channel(
                &map,
                "UV",
                "UVIndex",
                vertex_count,
                &mapping_counts,
                &mapping_offsets,
                &mappings,
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        }

        let mut vertex_colors: [Vec<[f32; 4]>; MAX_COLOR_SETS] =
            std::array::from_fn(|_| Vec::new());
        if let Some(el) = attrs.extract_case_insensitive("LayerElementColor") {
            let map = match child_attribute_map(el) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            vertex_colors[0] = match resolve_vec4_channel(
                &map,
                "Colors",
                "ColorIndex",
                vertex_count,
                &mapping_counts,
                &mapping_offsets,
                &mappings,
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        }

        let material_indices = if let Some(el) = attrs.extract_case_insensitive("LayerElementMaterial") {
            let map = match child_attribute_map(el) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            match read_vertex_data_materials(&map, &face_vertex_counts, vertex_count) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            }
        } else {
            Vec::new()
        };

        Ok(MeshGeometry {
            object: o,
            vertices,
            face_vertex_counts,
            normals,
            tangents,
            binormals,
            texture_coords,
            texture_coord_names,
            vertex_colors,
            material_indices,
        })
    }
}

/// Direct children of a geometry or layer-element subtree as attribute map (last duplicate key wins).
fn child_attribute_map(
    attr: &ElementAttribute,
) -> Result<HashMap<String, ElementAttribute>, FbxTryFromReason> {
    let ElementAttribute::SubTree(st) = attr else {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: "geometry".to_string(),
            detail: "expected SubTree attribute".to_string(),
        });
    };
    let arena = &st.amphitheatre;
    let root = arena.get(st.root_element_index).ok_or_else(|| FbxTryFromReason::InvalidAttributeFormat {
        name: "geometry".to_string(),
        detail: "subtree root missing".to_string(),
    })?;
    let mut map = HashMap::new();
    for &child_idx in &root.children {
        let Some(sub) = arena.extract_subtree(child_idx) else {
            continue;
        };
        let key = arena
            .get(child_idx)
            .map(|e| e.key.clone())
            .unwrap_or_default();
        map.insert(key, sub);
    }
    Ok(map)
}

fn dfs_collect_f32(
    arena: &ElementAmphitheatre,
    idx: usize,
    out: &mut Vec<f32>,
    ctx: &str,
) -> Result<(), FbxTryFromReason> {
    let el = arena.get(idx).ok_or_else(|| FbxTryFromReason::InvalidAttributeFormat {
        name: ctx.to_string(),
        detail: "element index out of range".to_string(),
    })?;
    out.extend(
        el.tokens
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<f32>().ok()),
    );
    for &ch in &el.children {
        dfs_collect_f32(arena, ch, out, ctx)?;
    }
    Ok(())
}

fn parse_f32_array(attr: &ElementAttribute, ctx: &str) -> Result<Vec<f32>, FbxTryFromReason> {
    let mut out = Vec::new();
    match attr {
        ElementAttribute::Leaf(_) => {
            out.extend(
                attr.get_tokens()
                    .iter()
                    .flat_map(|t| t.split(','))
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty())
                    .filter_map(|t| t.parse::<f32>().ok()),
            );
        }
        ElementAttribute::SubTree(st) => {
            dfs_collect_f32(&st.amphitheatre, st.root_element_index, &mut out, ctx)?;
        }
    }
    Ok(out)
}

fn dfs_collect_i32(
    arena: &ElementAmphitheatre,
    idx: usize,
    out: &mut Vec<i32>,
    ctx: &str,
) -> Result<(), FbxTryFromReason> {
    let el = arena.get(idx).ok_or_else(|| FbxTryFromReason::InvalidAttributeFormat {
        name: ctx.to_string(),
        detail: "element index out of range".to_string(),
    })?;
    out.extend(
        el.tokens
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<i32>().ok()),
    );
    for &ch in &el.children {
        dfs_collect_i32(arena, ch, out, ctx)?;
    }
    Ok(())
}

fn parse_i32_array(attr: &ElementAttribute, ctx: &str) -> Result<Vec<i32>, FbxTryFromReason> {
    let mut out = Vec::new();
    match attr {
        ElementAttribute::Leaf(_) => {
            out.extend(
                attr.get_tokens()
                    .iter()
                    .flat_map(|t| t.split(','))
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty())
                    .filter_map(|t| t.parse::<i32>().ok()),
            );
        }
        ElementAttribute::SubTree(st) => {
            dfs_collect_i32(&st.amphitheatre, st.root_element_index, &mut out, ctx)?;
        }
    }
    Ok(out)
}

fn vec3_positions_from_flat(f: &[f32], ctx: &str) -> Result<Vec<[f32; 3]>, FbxTryFromReason> {
    if f.len() % 3 != 0 {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: ctx.to_string(),
            detail: format!("vertex float count {} not divisible by 3", f.len()),
        });
    }
    Ok(f
        .chunks_exact(3)
        .map(|c| [c[0], c[1], c[2]])
        .collect())
}

/// Assimp [`MeshGeometry` ctor](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.cpp): expand to per-corner vertices and face sizes.
fn expand_mesh_polygon_vertices(
    temp_verts: &[[f32; 3]],
    temp_faces: &[i32],
) -> Result<(Vec<[f32; 3]>, Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>), FbxTryFromReason> {
    let vertex_count = temp_verts.len();
    let mut mapping_counts = vec![0u32; vertex_count];
    let mut expanded_vertices = Vec::new();
    let mut face_vertex_counts = Vec::new();
    let mut count = 0u32;

    for &index in temp_faces {
        let absi = if index < 0 {
            (-index - 1) as usize
        } else {
            index as usize
        };
        if absi >= vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: "PolygonVertexIndex".to_string(),
                detail: format!("index {absi} out of range (vertex count {vertex_count})"),
            });
        }
        expanded_vertices.push(temp_verts[absi]);
        count += 1;
        mapping_counts[absi] = mapping_counts[absi].saturating_add(1);
        if index < 0 {
            face_vertex_counts.push(count);
            count = 0;
        }
    }

    let polygon_vertex_count = expanded_vertices.len();
    let mut mapping_offsets = vec![0u32; vertex_count];
    let mut cursor = 0u32;
    for i in 0..vertex_count {
        mapping_offsets[i] = cursor;
        cursor += mapping_counts[i];
        mapping_counts[i] = 0;
    }

    let mut mappings = vec![0u32; polygon_vertex_count];
    cursor = 0;
    for &index in temp_faces {
        let absi = if index < 0 {
            (-index - 1) as usize
        } else {
            index as usize
        };
        let slot = mapping_offsets[absi] + mapping_counts[absi];
        mapping_counts[absi] += 1;
        mappings[slot as usize] = cursor;
        cursor += 1;
    }

    Ok((
        expanded_vertices,
        face_vertex_counts,
        mapping_counts,
        mapping_offsets,
        mappings,
    ))
}

fn vec3_from_flat_slice(data: &[f32], i: usize) -> Result<[f32; 3], FbxTryFromReason> {
    let base = i * 3;
    if base + 3 > data.len() {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: "Normals".to_string(),
            detail: "vec3 index out of range".to_string(),
        });
    }
    Ok([data[base], data[base + 1], data[base + 2]])
}

fn vec2_from_flat_slice(data: &[f32], i: usize) -> Result<[f32; 2], FbxTryFromReason> {
    let base = i * 2;
    if base + 2 > data.len() {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: "UV".to_string(),
            detail: "vec2 index out of range".to_string(),
        });
    }
    Ok([data[base], data[base + 1]])
}

fn vec4_from_flat_slice(data: &[f32], i: usize) -> Result<[f32; 4], FbxTryFromReason> {
    let base = i * 4;
    if base + 4 > data.len() {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: "Colors".to_string(),
            detail: "vec4 index out of range".to_string(),
        });
    }
    Ok([data[base], data[base + 1], data[base + 2], data[base + 3]])
}

fn resolve_vec3_channel(
    source: &HashMap<String, ElementAttribute>,
    data_name: &str,
    index_name: &str,
    vertex_count: usize,
    mapping_counts: &[u32],
    mapping_offsets: &[u32],
    mappings: &[u32],
) -> Result<Vec<[f32; 3]>, FbxTryFromReason> {
    let mapping_ty = source
        .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;
    let reference_ty = source
        .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;

    let mut is_direct = reference_ty.eq_ignore_ascii_case(REFERENCE_DIRECT);
    let mut is_index_to_direct = reference_ty.eq_ignore_ascii_case(REFERENCE_INDEX_TO_DIRECT);
    let has_data = source.extract_case_insensitive(data_name).is_some();
    let has_index = source.extract_case_insensitive(index_name).is_some();
    if is_index_to_direct && !has_index {
        is_direct = true;
        is_index_to_direct = false;
    }

    let empty = [0f32; 3];
    let mut data_out = vec![empty; vertex_count];

    if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE) && is_direct {
        if !has_data {
            return Ok(Vec::new());
        }
        let temp = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        if temp.len() != mapping_offsets.len() * 3 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: data_name.to_string(),
                detail: format!(
                    "{} {}: expected {} floats, got {}",
                    MAPPING_BY_VERTICE,
                    REFERENCE_DIRECT,
                    mapping_offsets.len() * 3,
                    temp.len()
                ),
            });
        }
        for i in 0..mapping_offsets.len() {
            let v = vec3_from_flat_slice(&temp, i)?;
            let istart = mapping_offsets[i] as usize;
            let iend = istart + mapping_counts[i] as usize;
            for j in istart..iend {
                data_out[mappings[j] as usize] = v;
            }
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE) && is_index_to_direct {
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        let temp_data = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        let uv_indices = parse_i32_array(
            source.extract_case_insensitive(index_name).unwrap(),
            index_name,
        )?;
        if uv_indices.len() != mapping_offsets.len() {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: index_name.to_string(),
                detail: format!("length mismatch for {MAPPING_BY_VERTICE}"),
            });
        }
        for i in 0..mapping_offsets.len() {
            let idx = uv_indices[i] as usize;
            let v = vec3_from_flat_slice(&temp_data, idx)?;
            let istart = mapping_offsets[i] as usize;
            let iend = istart + mapping_counts[i] as usize;
            for j in istart..iend {
                data_out[mappings[j] as usize] = v;
            }
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX) && is_direct {
        if !has_data {
            return Ok(Vec::new());
        }
        let temp = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        if temp.len() != vertex_count * 3 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: data_name.to_string(),
                detail: format!(
                    "{} {}: expected {} floats, got {}",
                    MAPPING_BY_POLYGON_VERTEX,
                    REFERENCE_DIRECT,
                    vertex_count * 3,
                    temp.len()
                ),
            });
        }
        for i in 0..vertex_count {
            data_out[i] = vec3_from_flat_slice(&temp, i)?;
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX) && is_index_to_direct {
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        let temp_data = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        let mut uv_indices = parse_i32_array(
            source.extract_case_insensitive(index_name).unwrap(),
            index_name,
        )?;
        if uv_indices.len() > vertex_count {
            uv_indices.truncate(vertex_count);
        }
        if uv_indices.len() != vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: index_name.to_string(),
                detail: format!(
                    "{} {}: expected {} indices, got {}",
                    MAPPING_BY_POLYGON_VERTEX,
                    REFERENCE_INDEX_TO_DIRECT,
                    vertex_count,
                    uv_indices.len()
                ),
            });
        }
        for (next, &i) in uv_indices.iter().enumerate() {
            if i == -1 {
                data_out[next] = empty;
                continue;
            }
            let ui = i as usize;
            if ui * 3 + 3 > temp_data.len() {
                return Err(FbxTryFromReason::InvalidAttributeFormat {
                    name: data_name.to_string(),
                    detail: "index out of range".to_string(),
                });
            }
            data_out[next] = vec3_from_flat_slice(&temp_data, ui)?;
        }
    } else {
        return Ok(Vec::new());
    }

    Ok(data_out)
}

fn resolve_vec2_channel(
    source: &HashMap<String, ElementAttribute>,
    data_name: &str,
    index_name: &str,
    vertex_count: usize,
    mapping_counts: &[u32],
    mapping_offsets: &[u32],
    mappings: &[u32],
) -> Result<Vec<[f32; 2]>, FbxTryFromReason> {
    let mapping_ty = source
        .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;
    let reference_ty = source
        .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;

    let mut is_direct = reference_ty.eq_ignore_ascii_case(REFERENCE_DIRECT);
    let mut is_index_to_direct = reference_ty.eq_ignore_ascii_case(REFERENCE_INDEX_TO_DIRECT);
    let has_data = source.extract_case_insensitive(data_name).is_some();
    let has_index = source.extract_case_insensitive(index_name).is_some();
    if is_index_to_direct && !has_index {
        is_direct = true;
        is_index_to_direct = false;
    }

    let empty = [0f32; 2];
    let mut data_out = vec![empty; vertex_count];

    if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE) && is_direct {
        if !has_data {
            return Ok(Vec::new());
        }
        let temp = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        if temp.len() != mapping_offsets.len() * 2 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: data_name.to_string(),
                detail: format!("length mismatch {MAPPING_BY_VERTICE} vec2"),
            });
        }
        for i in 0..mapping_offsets.len() {
            let v = vec2_from_flat_slice(&temp, i)?;
            let istart = mapping_offsets[i] as usize;
            let iend = istart + mapping_counts[i] as usize;
            for j in istart..iend {
                data_out[mappings[j] as usize] = v;
            }
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE) && is_index_to_direct {
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        let temp_data = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        let uv_indices = parse_i32_array(
            source.extract_case_insensitive(index_name).unwrap(),
            index_name,
        )?;
        if uv_indices.len() != mapping_offsets.len() {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: index_name.to_string(),
                detail: "length mismatch".to_string(),
            });
        }
        for i in 0..mapping_offsets.len() {
            let idx = uv_indices[i] as usize;
            let v = vec2_from_flat_slice(&temp_data, idx)?;
            let istart = mapping_offsets[i] as usize;
            let iend = istart + mapping_counts[i] as usize;
            for j in istart..iend {
                data_out[mappings[j] as usize] = v;
            }
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX) && is_direct {
        if !has_data {
            return Ok(Vec::new());
        }
        let temp = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        if temp.len() != vertex_count * 2 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: data_name.to_string(),
                detail: format!("length mismatch {MAPPING_BY_POLYGON_VERTEX} vec2"),
            });
        }
        for i in 0..vertex_count {
            data_out[i] = vec2_from_flat_slice(&temp, i)?;
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX) && is_index_to_direct {
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        let temp_data = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        let mut uv_indices = parse_i32_array(
            source.extract_case_insensitive(index_name).unwrap(),
            index_name,
        )?;
        if uv_indices.len() > vertex_count {
            uv_indices.truncate(vertex_count);
        }
        if uv_indices.len() != vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: index_name.to_string(),
                detail: format!(
                    "length mismatch {MAPPING_BY_POLYGON_VERTEX} {REFERENCE_INDEX_TO_DIRECT} vec2"
                ),
            });
        }
        for (next, &i) in uv_indices.iter().enumerate() {
            if i == -1 {
                data_out[next] = empty;
                continue;
            }
            let ui = i as usize;
            data_out[next] = vec2_from_flat_slice(&temp_data, ui)?;
        }
    } else {
        return Ok(Vec::new());
    }

    Ok(data_out)
}

fn resolve_vec4_channel(
    source: &HashMap<String, ElementAttribute>,
    data_name: &str,
    index_name: &str,
    vertex_count: usize,
    mapping_counts: &[u32],
    mapping_offsets: &[u32],
    mappings: &[u32],
) -> Result<Vec<[f32; 4]>, FbxTryFromReason> {
    let mapping_ty = source
        .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;
    let reference_ty = source
        .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;

    let mut is_direct = reference_ty.eq_ignore_ascii_case(REFERENCE_DIRECT);
    let mut is_index_to_direct = reference_ty.eq_ignore_ascii_case(REFERENCE_INDEX_TO_DIRECT);
    let has_data = source.extract_case_insensitive(data_name).is_some();
    let has_index = source.extract_case_insensitive(index_name).is_some();
    if is_index_to_direct && !has_index {
        is_direct = true;
        is_index_to_direct = false;
    }

    let empty = [0f32; 4];
    let mut data_out = vec![empty; vertex_count];

    if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE) && is_direct {
        if !has_data {
            return Ok(Vec::new());
        }
        let temp = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        if temp.len() != mapping_offsets.len() * 4 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: data_name.to_string(),
                detail: format!("length mismatch vec4 {MAPPING_BY_VERTICE}"),
            });
        }
        for i in 0..mapping_offsets.len() {
            let v = vec4_from_flat_slice(&temp, i)?;
            let istart = mapping_offsets[i] as usize;
            let iend = istart + mapping_counts[i] as usize;
            for j in istart..iend {
                data_out[mappings[j] as usize] = v;
            }
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE) && is_index_to_direct {
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        let temp_data = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        let ix = parse_i32_array(
            source.extract_case_insensitive(index_name).unwrap(),
            index_name,
        )?;
        if ix.len() != mapping_offsets.len() {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: index_name.to_string(),
                detail: "length mismatch".to_string(),
            });
        }
        for i in 0..mapping_offsets.len() {
            let v = vec4_from_flat_slice(&temp_data, ix[i] as usize)?;
            let istart = mapping_offsets[i] as usize;
            let iend = istart + mapping_counts[i] as usize;
            for j in istart..iend {
                data_out[mappings[j] as usize] = v;
            }
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX) && is_direct {
        if !has_data {
            return Ok(Vec::new());
        }
        let temp = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        if temp.len() != vertex_count * 4 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: data_name.to_string(),
                detail: format!("length mismatch vec4 {MAPPING_BY_POLYGON_VERTEX}"),
            });
        }
        for i in 0..vertex_count {
            data_out[i] = vec4_from_flat_slice(&temp, i)?;
        }
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX) && is_index_to_direct {
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        let temp_data = parse_f32_array(
            source.extract_case_insensitive(data_name).unwrap(),
            data_name,
        )?;
        let mut ix = parse_i32_array(
            source.extract_case_insensitive(index_name).unwrap(),
            index_name,
        )?;
        if ix.len() > vertex_count {
            ix.truncate(vertex_count);
        }
        if ix.len() != vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: index_name.to_string(),
                detail: "length mismatch vec4".to_string(),
            });
        }
        for (next, &i) in ix.iter().enumerate() {
            if i == -1 {
                data_out[next] = empty;
                continue;
            }
            data_out[next] = vec4_from_flat_slice(&temp_data, i as usize)?;
        }
    } else {
        return Ok(Vec::new());
    }

    Ok(data_out)
}

/// Assimp `ReadVertexDataMaterials` (subset): `AllSame` and `ByPolygon` + `IndexToDirect`.
fn read_vertex_data_materials(
    source: &HashMap<String, ElementAttribute>,
    face_vertex_counts: &[u32],
    polygon_vertex_count: usize,
) -> Result<Vec<i32>, FbxTryFromReason> {
    let face_count = face_vertex_counts.len();
    if face_count == 0 {
        return Ok(Vec::new());
    }
    let mapping_ty = source
        .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;
    let reference_ty = source
        .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
        .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))?;

    let Some(mat_el) = source.extract_case_insensitive("Materials") else {
        return Ok(Vec::new());
    };
    let materials_out = parse_i32_array(mat_el, "Materials")?;

    if mapping_ty.eq_ignore_ascii_case(MAPPING_ALL_SAME) {
        if materials_out.is_empty() {
            return Ok(Vec::new());
        }
        let count_neg = materials_out.iter().filter(|&&n| n < 0).count();
        if count_neg == materials_out.len() {
            return Ok(Vec::new());
        }
        let v = materials_out[0];
        Ok(vec![v; polygon_vertex_count])
    } else if mapping_ty.eq_ignore_ascii_case(MAPPING_BY_POLYGON)
        && reference_ty.eq_ignore_ascii_case(REFERENCE_INDEX_TO_DIRECT)
    {
        if materials_out.len() != face_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: "Materials".to_string(),
                detail: format!(
                    "{}: expected {} material indices, got {}",
                    MAPPING_BY_POLYGON,
                    face_count,
                    materials_out.len()
                ),
            });
        }
        let count_neg = materials_out.iter().filter(|&&n| n < 0).count();
        if count_neg == materials_out.len() {
            return Ok(Vec::new());
        }
        let mut per_corner = Vec::with_capacity(polygon_vertex_count);
        for (&m, &n) in materials_out.iter().zip(face_vertex_counts.iter()) {
            for _ in 0..n {
                per_corner.push(m);
            }
        }
        if per_corner.len() != polygon_vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: "Materials".to_string(),
                detail: format!(
                    "expanded material indices length {} != polygon vertex count {}",
                    per_corner.len(),
                    polygon_vertex_count
                ),
            });
        }
        Ok(per_corner)
    } else {
        Ok(Vec::new())
    }
}