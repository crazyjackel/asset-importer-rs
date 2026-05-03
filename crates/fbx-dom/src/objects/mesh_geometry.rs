//! FBX `Geometry` / `Mesh` — Assimp [`MeshGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.h) / [`FBXMeshGeometry.cpp`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.cpp).
//!
//! ## Layout
//!
//! - **Control points** — `Vertices` / `PolygonVertexIndex` on the geometry root; ASCII stores large
//!   arrays as `Name: *N { a: … }`; `parse_f32_array` / `parse_i32_array` read the optional **`a`**
//!   child or fall back to tokens on the node (unit tests).
//! - **Layers** — `LayerElementNormal`, `LayerElementUV`, … each hold `MappingInformationType`,
//!   `ReferenceInformationType`, and channel arrays. `expand_mesh_polygon_vertices` duplicates
//!   positions per corner and builds `mapping_*` tables for `resolve_flat_f32_channel`.

use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::ParseFloatError;
use std::num::ParseIntError;

use crate::OwnedObject;
use fbxscii::ElementAttribute;

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

const ATTR_VERTICES: &str = "Vertices";
const ATTR_POLYGON_VERTEX_INDEX: &str = "PolygonVertexIndex";
const ATTR_LAYER_ELEMENT_NORMAL: &str = "LayerElementNormal";
const ATTR_LAYER_ELEMENT_TANGENT: &str = "LayerElementTangent";
const ATTR_LAYER_ELEMENT_BINORMAL: &str = "LayerElementBinormal";
const ATTR_LAYER_ELEMENT_UV: &str = "LayerElementUV";
const ATTR_MATERIALS: &str = "Materials";

const ACCESSOR_KEY: &str = "a";

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
        // Check if tagged as MeshGeometry
        match fbx_object_tag(&o) {
            FbxObjectTag::MeshGeometry => {}
            _ => {
                return Err(FbxTypeMismatch::wrong_object_kind(
                    o,
                    "MeshGeometry".to_string(),
                ));
            }
        }

        let attrs = &o.attributes;

        // Extract Vertices (ASCII: `Vertices: *N { a: ... }`; tests may use a leaf with tokens only).
        let verts_attr = match attrs.extract_case_insensitive(ATTR_VERTICES) {
            Some(a) => a,
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: ATTR_VERTICES.to_string(),
                    },
                ));
            }
        };
        // Parse Vertices
        let vertices_result = parse_f32_array(verts_attr);
        let Ok(vertices) = vertices_result else {
            return Err(FbxTypeMismatch::new(o, FbxTryFromReason::InvalidAttributeFormat {
                name: ATTR_VERTICES.to_string(),
                detail: format!("invalid float token: {}", vertices_result.unwrap_err()),
            }));
        };
        let vertices = vertices
            .chunks_exact(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect::<Vec<[f32; 3]>>();

        // Extract Face Indices
        let poly_attr = match attrs.extract_case_insensitive(ATTR_POLYGON_VERTEX_INDEX) {
            Some(a) => a,
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: ATTR_POLYGON_VERTEX_INDEX.to_string(),
                    },
                ));
            }
        };
        let temp_faces_result = parse_i32_array(poly_attr);
        let Ok(temp_faces) = temp_faces_result else {
            return Err(FbxTypeMismatch::new(o, FbxTryFromReason::InvalidAttributeFormat {
                name: ATTR_POLYGON_VERTEX_INDEX.to_string(),
                detail: format!("invalid int token: {}", temp_faces_result.unwrap_err()),
            }));
        };

        let (vertices, face_vertex_counts, mapping_counts, mapping_offsets, mappings) =
            match expand_mesh_polygon_vertices(&vertices, &temp_faces) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
        let vertex_count = vertices.len();

        let mut normals = Vec::new();
        if let Some(el) = attrs.extract_case_insensitive(ATTR_LAYER_ELEMENT_NORMAL) {
            let map = el.get_children_distinct();
            let mapping_ty = match map
                .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let reference_ty = match map
                .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };

            let normals_flat = match resolve_flat_f32_channel(
                &map,
                ResolveFlatF32ChannelParams {
                    data_name: "Normals",
                    index_name: "NormalsIndex",
                    vertex_count,
                    components: 3,
                    mapping_counts: &mapping_counts,
                    mapping_offsets: &mapping_offsets,
                    mappings: &mappings,
                    mapping_ty: mapping_ty,
                    reference_ty: reference_ty,
                },
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            normals = normals_flat
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect();
        }

        let mut tangents = Vec::new();
        if let Some(el) = attrs.extract_case_insensitive(ATTR_LAYER_ELEMENT_TANGENT) {
            let map = el.get_children_distinct();
            let (data_name, index_name) = if map.extract_case_insensitive("Tangents").is_some() {
                ("Tangents", "TangentsIndex")
            } else {
                ("Tangent", "TangentIndex")
            };
            let mapping_ty = match map
                .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let reference_ty = match map
                .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let tangents_flat = match resolve_flat_f32_channel(
                &map,
                ResolveFlatF32ChannelParams {
                    data_name,
                    index_name,
                    vertex_count,
                    components: 3,
                    mapping_counts: &mapping_counts,
                    mapping_offsets: &mapping_offsets,
                    mappings: &mappings,
                    mapping_ty: mapping_ty,
                    reference_ty: reference_ty,
                },
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            tangents = tangents_flat
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect();
        }

        let mut binormals = Vec::new();
        if let Some(el) = attrs.extract_case_insensitive(ATTR_LAYER_ELEMENT_BINORMAL) {
            let map = el.get_children_distinct();
            let (data_name, index_name) = if map.extract_case_insensitive("Binormals").is_some() {
                ("Binormals", "BinormalsIndex")
            } else {
                ("Binormal", "BinormalIndex")
            };
            let mapping_ty = match map
                .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let reference_ty = match map
                .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let binormals_flat = match resolve_flat_f32_channel(
                &map,
                ResolveFlatF32ChannelParams {
                    data_name,
                    index_name,
                    vertex_count,
                    components: 3,
                    mapping_counts: &mapping_counts,
                    mapping_offsets: &mapping_offsets,
                    mappings: &mappings,
                    mapping_ty: mapping_ty,
                    reference_ty: reference_ty,
                },
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            binormals = binormals_flat
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect();
        }

        let mut texture_coords: [Vec<[f32; 2]>; MAX_UV_CHANNELS] = Default::default();
        let mut texture_coord_names: [String; MAX_UV_CHANNELS] = Default::default();

        if let Some(el) = attrs.extract_case_insensitive(ATTR_LAYER_ELEMENT_UV) {
            let map = el.get_children_distinct();
            if let Ok(Some(name)) = map.optional_token_case_insensitive("Name") {
                texture_coord_names[0] = name
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'')
                    .to_string();
            }
            let mapping_ty = match map
                .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let reference_ty = match map
                .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let uv_flat = match resolve_flat_f32_channel(
                &map,
                ResolveFlatF32ChannelParams {
                    data_name: "UV",
                    index_name: "UVIndex",
                    vertex_count,
                    components: 2,
                    mapping_counts: &mapping_counts,
                    mapping_offsets: &mapping_offsets,
                    mappings: &mappings,
                    mapping_ty: mapping_ty,
                    reference_ty: reference_ty,
                },
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            texture_coords[0] = uv_flat.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
        }

        let mut vertex_colors: [Vec<[f32; 4]>; MAX_COLOR_SETS] = Default::default();
        if let Some(el) = attrs.extract_case_insensitive("LayerElementColor") {
            let map = el.get_children_distinct();
            let mapping_ty = match map
                .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let reference_ty = match map
                .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
            {
                Ok(s) => s,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            let colors_flat = match resolve_flat_f32_channel(
                &map,
                ResolveFlatF32ChannelParams {
                    data_name: "Colors",
                    index_name: "ColorIndex",
                    vertex_count,
                    components: 4,
                    mapping_counts: &mapping_counts,
                    mapping_offsets: &mapping_offsets,
                    mappings: &mappings,
                    mapping_ty: mapping_ty,
                    reference_ty: reference_ty,
                },
            ) {
                Ok(v) => v,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            };
            vertex_colors[0] = colors_flat
                .chunks_exact(4)
                .map(|c| [c[0], c[1], c[2], c[3]])
                .collect();
        }

        let material_indices =
            if let Some(el) = attrs.extract_case_insensitive("LayerElementMaterial") {
                let map = el.get_children_distinct();
                let mapping_ty = match map
                    .require_token_case_insensitive(ATTR_MAPPING_INFORMATION_TYPE)
                    .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
                {
                    Ok(s) => s,
                    Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
                };
                let reference_ty = match map
                    .require_token_case_insensitive(ATTR_REFERENCE_INFORMATION_TYPE)
                    .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\''))
                {
                    Ok(s) => s,
                    Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
                };
                match read_vertex_data_materials(
                    &map,
                    &face_vertex_counts,
                    vertex_count,
                    &mapping_ty,
                    &reference_ty,
                ) {
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

/// Comma-separated float list from `attr` tokens, after optional ASCII **`a:`** child (see `ACCESSOR_KEY`).
fn parse_f32_array(attr: &ElementAttribute) -> Result<Vec<f32>, ParseFloatError> {
    let children = attr.get_children_distinct();
    let payload = children.get(ACCESSOR_KEY).unwrap_or(attr);
    let tokens = payload.get_tokens();
    tokens
        .iter()
        .flat_map(|t| t.split(','))
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<f32>())
        .collect()
}

/// Comma-separated `i32` list; same `a:` drill-down as [`parse_f32_array`].
fn parse_i32_array(attr: &ElementAttribute) -> Result<Vec<i32>, ParseIntError> {
    let children = attr.get_children_distinct();
    let payload = children.get(ACCESSOR_KEY).unwrap_or(attr);
    let tokens = payload.get_tokens();
    tokens
        .iter()
        .flat_map(|t| t.split(','))
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<i32>())
        .collect()
}

/// Expand FBX mesh indices into a **per-polygon-vertex** (“corner”) linear layout and build tables to
/// remap **per-corner** data from FBX’s indexed vertex pool.
///
/// Mirrors Assimp [`FBXMeshGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.cpp)
/// (`MeshGeometry` ctor after `ParseVectorDataArray`).
///
/// ## FBX encoding (`PolygonVertexIndex`)
///
/// Indices reference rows in `Vertices` (the control-point / vertex pool). The **last** index of each
/// polygon is **negative**; its absolute value is still `absi = (-index - 1)`, marking the end of that
/// face. Non-final corners use non-negative indices. Example triangle `0, 1, -3` uses pool vertices
/// 0, 1, and 2 (`-3` → abs index 2).
///
/// ## Why expand?
///
/// Layer data (normals, UVs, …) is often stored **ByPolygonVertex** or **ByVertice** with a different
/// indexing than the raw pool. Assimp duplicates pool positions so each **corner** gets its own row in
/// `expanded_vertices` (length = number of polygon corners = `temp_faces.len()` in well-formed files).
/// We must also know, for each pool index `i`, which contiguous slice of “corner slots” belongs to
/// that pool vertex so channels can be scattered into the expanded order.
///
/// ## Outputs
///
/// - **`expanded_vertices`**: `temp_verts[absi]` appended once per corner, in `temp_faces` order
///   (ignoring sign on the last index of each face).
/// - **`face_vertex_counts`**: number of corners per polygon (from running `count` reset on each
///   negative index).
/// - **`mapping_offsets[i]`**: start offset in corner space for pool vertex `i` (prefix sum of how
///   many corners reference pool vertex `i`).
/// - **`mapping_counts`**: after this function returns, again the number of corners referencing each
///   pool vertex `i` (same as after pass 1; rebuilt during pass 3).
/// - **`mappings`**: length = corner count. For each corner in `temp_faces` order (global corner
///   index `cursor`), `mappings[slot] = cursor` where `slot` is the next free slot in the slice
///   `[mapping_offsets[absi] ..)` reserved for pool vertex `absi`. So `mappings` ties pool-vertex
///   corner slots to the global expanded corner index for channel gather/scatter in
///   [`resolve_flat_f32_channel`].
///
/// ## Three passes (same as Assimp)
///
/// 1. **Walk `temp_faces` once**: append expanded positions; increment `mapping_counts[absi]` per
///    reference; accumulate polygon sizes into `face_vertex_counts` when hitting a negative index.
/// 2. **Prefix-sum `mapping_counts` → `mapping_offsets`**, then zero `mapping_counts` (slots will be
///    filled in pass 3).
/// 3. **Walk `temp_faces` again**: for each corner, assign `mappings[mapping_offsets[absi] +
///    mapping_counts[absi]++] = global_corner_index` so each pool vertex’s corners occupy a stable,
///    contiguous block in corner index space.
fn expand_mesh_polygon_vertices(
    temp_verts: &[[f32; 3]],
    temp_faces: &[i32],
) -> Result<(Vec<[f32; 3]>, Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>), FbxTryFromReason> {
    let vertex_count = temp_verts.len();
    let mut mapping_counts = vec![0u32; vertex_count];
    let mut expanded_vertices = Vec::new();
    let mut face_vertex_counts = Vec::new();
    let mut count = 0u32;

    // Pass 1 — see module-level docs on [`expand_mesh_polygon_vertices`].
    for &index in temp_faces {
        // Get absolute index
        let absi = if index < 0 {
            (-index - 1) as usize
        } else {
            index as usize
        };
        if absi >= vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: ATTR_POLYGON_VERTEX_INDEX.to_string(),
                detail: format!("index {absi} out of range (vertex count {vertex_count})"),
            });
        }
        // Add vertex to expanded vertices
        expanded_vertices.push(temp_verts[absi]);
        count += 1;
        // Update the running count of how many vertices are expanded.
        mapping_counts[absi] = mapping_counts[absi].saturating_add(1);
        if index < 0 {
            // Count the index difference between this and the last negative index.
            face_vertex_counts.push(count);
            count = 0;
        }
    }

    let polygon_vertex_count = expanded_vertices.len();
    // Pass 2 — prefix sums into mapping_offsets; clear mapping_counts for pass 3 slot filling.
    let mut mapping_offsets = vec![0u32; vertex_count];
    let mut cursor = 0u32;
    for i in 0..vertex_count {
        mapping_offsets[i] = cursor;
        cursor += mapping_counts[i];
        mapping_counts[i] = 0;
    }

    // Pass 3 — for each corner in order, assign stable slot in per-pool-vertex ranges (see doc above).
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

pub struct ResolveFlatF32ChannelParams<'a> {
    data_name: &'a str,
    index_name: &'a str,
    vertex_count: usize,
    components: usize,
    mapping_counts: &'a [u32],
    mapping_offsets: &'a [u32],
    mappings: &'a [u32],
    mapping_ty: &'a str,
    reference_ty: &'a str,
}

fn resolve_flat_f32_channel(
    source: &HashMap<String, ElementAttribute>,
    params: ResolveFlatF32ChannelParams<'_>,
) -> Result<Vec<f32>, FbxTryFromReason> {
    let mut is_direct = params.reference_ty.eq_ignore_ascii_case(REFERENCE_DIRECT);
    let mut is_index_to_direct = params
        .reference_ty
        .eq_ignore_ascii_case(REFERENCE_INDEX_TO_DIRECT);
    let has_data = source.extract_case_insensitive(params.data_name).is_some();
    let has_index = source.extract_case_insensitive(params.index_name).is_some();
    if is_index_to_direct && !has_index {
        is_direct = true;
        is_index_to_direct = false;
    }

    if params.components == 0 {
        return Ok(Vec::new());
    }
    let mut vertex_out = vec![0f32; params.vertex_count * params.components];

    let by_vertice = params.mapping_ty.eq_ignore_ascii_case(MAPPING_BY_VERTICE);
    let by_polygon_vertex = params
        .mapping_ty
        .eq_ignore_ascii_case(MAPPING_BY_POLYGON_VERTEX);

    if by_vertice && is_direct {
        // Case 1: Map type is ByVertice and reference type is Direct
        if !has_data {
            return Ok(Vec::new());
        }
        let channel_attribute = source.extract_case_insensitive(params.data_name).ok_or(
            FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: "data channel not found".to_string(),
            },
        )?;
        let channel_data_result = parse_f32_array(&channel_attribute);
        let Ok(channel_data) = channel_data_result else {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: format!("invalid float token: {}", channel_data_result.unwrap_err()),
            });
        };
        if channel_data.len() != params.mapping_offsets.len() * params.components {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: format!(
                    "{} {}: expected {} floats, got {}",
                    MAPPING_BY_VERTICE,
                    REFERENCE_DIRECT,
                    params.mapping_offsets.len() * params.components,
                    channel_data.len()
                ),
            });
        }
        for i in 0..params.mapping_offsets.len() {
            let istart = params.mapping_offsets[i] as usize;
            let iend = istart + params.mapping_counts[i] as usize;
            for j in istart..iend {
                vertex_out[params.mappings[j] as usize] = channel_data[i];
            }
        }
    } else if by_vertice && is_index_to_direct {
        // Case 2: Map type is ByVertice and reference type is IndexToDirect
        if !has_data || !has_index {
            return Ok(Vec::new());
        }

        let channel_attribute = source.extract_case_insensitive(params.data_name).ok_or(
            FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: "data channel not found".to_string(),
            },
        )?;
        let channel_data_result = parse_f32_array(&channel_attribute);
        let Ok(channel_data) = channel_data_result else {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: format!("invalid float token: {}", channel_data_result.unwrap_err()),
            });
        };
        let channel_index_attribute = source.extract_case_insensitive(params.index_name).ok_or(
            FbxTryFromReason::InvalidAttributeFormat {
                name: params.index_name.to_string(),
                detail: "index channel not found".to_string(),
            },
        )?;
        let channel_index_data_result = parse_i32_array(&channel_index_attribute);
        let Ok(channel_index_data) = channel_index_data_result else {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.index_name.to_string(),
                detail: format!("invalid int token: {}", channel_index_data_result.unwrap_err()),
            });
        };
        if channel_index_data.len() != params.mapping_offsets.len() {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.index_name.to_string(),
                detail: format!("length mismatch for {MAPPING_BY_VERTICE}"),
            });
        }
        // Remap the vertex data to the new vertex positions, based on the index data.
        for i in 0..params.mapping_offsets.len() {
            let idx = channel_index_data[i] as usize;
            let istart = params.mapping_offsets[i] as usize;
            let iend = istart + params.mapping_counts[i] as usize;
            for j in istart..iend {
                vertex_out[params.mappings[j] as usize] = channel_data[idx];
            }
        }
    } else if by_polygon_vertex && is_direct {
        // Case 3: Map type is ByPolygonVertex and reference type is Direct
        if !has_data {
            return Ok(Vec::new());
        }
        let channel_attribute = source.extract_case_insensitive(params.data_name).ok_or(
            FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: "data channel not found".to_string(),
            },
        )?;
        let channel_data_result = parse_f32_array(&channel_attribute);
        let Ok(channel_data) = channel_data_result else {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: format!("invalid float token: {}", channel_data_result.unwrap_err()),
            });
        };
        if channel_data.len() != params.vertex_count * params.components {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: format!(
                    "{} {}: expected {} floats, got {}",
                    MAPPING_BY_POLYGON_VERTEX,
                    REFERENCE_DIRECT,
                    params.vertex_count,
                    channel_data.len()
                ),
            });
        }
        // Copy the channel data to the vertex output.
        vertex_out = channel_data;
    } else if by_polygon_vertex && is_index_to_direct {
        // Case 4: Map type is ByPolygonVertex and reference type is IndexToDirect
        if !has_data || !has_index {
            return Ok(Vec::new());
        }
        // Get data and attributes for the data and index keys.
        let channel_attribute = source.extract_case_insensitive(params.data_name).ok_or(
            FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: "data channel not found".to_string(),
            },
        )?;
        let channel_data_result = parse_f32_array(&channel_attribute);
        let Ok(channel_data) = channel_data_result else {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.data_name.to_string(),
                detail: format!("invalid float token: {}", channel_data_result.unwrap_err()),
            });
        };
        let channel_index_attribute = source.extract_case_insensitive(params.index_name).ok_or(
            FbxTryFromReason::InvalidAttributeFormat {
                name: params.index_name.to_string(),
                detail: "index channel not found".to_string(),
            },
        )?;
        let channel_index_data_result = parse_i32_array(&channel_index_attribute);
        let Ok(mut channel_index_data) = channel_index_data_result else {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.index_name.to_string(),
                detail: format!("invalid int token: {}", channel_index_data_result.unwrap_err()),
            });
        };
        // Truncate the index data if it's longer than the vertex count.
        if channel_index_data.len() > params.vertex_count {
            channel_index_data.truncate(params.vertex_count);
        }
        // Make sure the index data matches the vertex count.
        if channel_index_data.len() != params.vertex_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: params.index_name.to_string(),
                detail: format!(
                    "{} {}: expected {} indices, got {}",
                    MAPPING_BY_POLYGON_VERTEX,
                    REFERENCE_INDEX_TO_DIRECT,
                    params.vertex_count,
                    channel_index_data.len()
                ),
            });
        }
        let mut next_index = 0;
        for &i in channel_index_data.iter() {
            if i == -1 {
                vertex_out[next_index] = 0f32;
                next_index += 1;
                continue;
            }
            vertex_out[next_index] = channel_data[i as usize];
            next_index += 1;
        }
    } else {
        // Case 5: Map type is not ByVertice or ByPolygonVertex, or reference type is not Direct or IndexToDirect
        return Ok(Vec::new());
    }
    Ok(vertex_out)
}

/// Assimp `ReadVertexDataMaterials` (subset): `AllSame` and `ByPolygon` + `IndexToDirect`.
fn read_vertex_data_materials(
    source: &HashMap<String, ElementAttribute>,
    face_vertex_counts: &[u32],
    polygon_vertex_count: usize,
    mapping_ty: &str,
    reference_ty: &str,
) -> Result<Vec<i32>, FbxTryFromReason> {
    // Face count guard
    let face_count = face_vertex_counts.len();
    if face_count == 0 {
        return Ok(Vec::new());
    }

    // Extract Materials
    let Some(mat_el) = source.extract_case_insensitive("Materials") else {
        return Ok(Vec::new());
    };
    let materials_out_result = parse_i32_array(mat_el);
    let Ok(materials_out) = materials_out_result else {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: ATTR_MATERIALS.to_string(),
            detail: format!("invalid int token: {}", materials_out_result.unwrap_err()),
        });
    };

    if mapping_ty.eq_ignore_ascii_case(MAPPING_ALL_SAME) {
        // Case 1: Map type is AllSame
        // All materials are the same, so return a mapping of all polygons to the same material.
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
        // Case 2: Map type is ByPolygon and reference type is IndexToDirect
        // The materials are indexed by the face index, so we need to map the materials to the vertices.
        if materials_out.len() != face_count {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: ATTR_MATERIALS.to_string(),
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
                name: ATTR_MATERIALS.to_string(),
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{
        Element, ElementAmphitheatre, ElementAttribute, LeafAttribute, SubTreeAttribute,
    };

    use crate::OwnedObject;

    use super::super::{
        FbxTryFromReason, GEOMETRY_LINE_CLASS_NAME, GEOMETRY_MESH_CLASS_NAME, GEOMETRY_TYPE_NAME,
    };
    use super::MeshGeometry;

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    fn append_layer_string_child(
        arena: &mut ElementAmphitheatre,
        root_idx: usize,
        key: &str,
        token: &str,
    ) {
        let mut el = Element::new(key.to_string());
        el.tokens = vec![token.to_string()];
        el.parent_index = Some(root_idx);
        let idx = arena.insert(el);
        arena.get_mut(root_idx).unwrap().children.push(idx);
    }

    /// Same nesting as ASCII FBX (e.g. `duck.fbx`): mapping + reference under the layer root, then data.
    fn layer_element_normal(mapping: &str, reference: &str, normals_csv: &str) -> ElementAttribute {
        let mut arena = ElementAmphitheatre::new();
        let root_idx = arena.insert(Element::new("LayerElementNormal".into()));
        append_layer_string_child(
            &mut arena,
            root_idx,
            super::ATTR_MAPPING_INFORMATION_TYPE,
            mapping,
        );
        append_layer_string_child(
            &mut arena,
            root_idx,
            super::ATTR_REFERENCE_INFORMATION_TYPE,
            reference,
        );
        let mut normals_el = Element::new("Normals".into());
        normals_el.tokens = vec![normals_csv.to_string()];
        normals_el.parent_index = Some(root_idx);
        let normals_idx = arena.insert(normals_el);
        arena.get_mut(root_idx).unwrap().children.push(normals_idx);

        ElementAttribute::SubTree(Box::new(SubTreeAttribute {
            amphitheatre: arena,
            root_element_index: root_idx,
        }))
    }

    fn layer_element_material(
        mapping: &str,
        reference: &str,
        materials_csv: &str,
    ) -> ElementAttribute {
        let mut arena = ElementAmphitheatre::new();
        let root_idx = arena.insert(Element::new("LayerElementMaterial".into()));
        append_layer_string_child(
            &mut arena,
            root_idx,
            super::ATTR_MAPPING_INFORMATION_TYPE,
            mapping,
        );
        append_layer_string_child(
            &mut arena,
            root_idx,
            super::ATTR_REFERENCE_INFORMATION_TYPE,
            reference,
        );
        let mut mats_el = Element::new(super::ATTR_MATERIALS.into());
        mats_el.tokens = vec![materials_csv.to_string()];
        mats_el.parent_index = Some(root_idx);
        let mats_idx = arena.insert(mats_el);
        arena.get_mut(root_idx).unwrap().children.push(mats_idx);

        ElementAttribute::SubTree(Box::new(SubTreeAttribute {
            amphitheatre: arena,
            root_element_index: root_idx,
        }))
    }

    fn owned_mesh(attrs: HashMap<String, ElementAttribute>) -> OwnedObject {
        OwnedObject {
            object_index: 11,
            name: "Geometry::TestMesh".into(),
            type_name: GEOMETRY_TYPE_NAME.into(),
            class_name: GEOMETRY_MESH_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        }
    }

    fn minimal_base_attrs() -> HashMap<String, ElementAttribute> {
        let mut attrs = HashMap::new();
        attrs.insert("Vertices".into(), leaf(&["0,0,0,1,0,0,0,1,0"]));
        attrs.insert("PolygonVertexIndex".into(), leaf(&["0,1,-3"]));
        attrs
    }

    #[test]
    fn minimal_triangle_expands_corners() {
        let attrs = minimal_base_attrs();
        let mesh = MeshGeometry::try_from(owned_mesh(attrs)).unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.face_vertex_counts, vec![3u32]);
        assert!(mesh.normals.is_empty());
        assert!(mesh.material_indices.is_empty());
    }

    #[test]
    fn mapping_and_reference_trim_quotes_and_lowercase_keys() {
        let mut attrs = minimal_base_attrs();
        let mut arena = ElementAmphitheatre::new();
        let root_idx = arena.insert(Element::new("LayerElementNormal".into()));
        append_layer_string_child(
            &mut arena,
            root_idx,
            "mappinginformationtype",
            "  \"ByPolygonVertex\"  ",
        );
        append_layer_string_child(&mut arena, root_idx, "referenceinformationtype", "'Direct'");
        let mut normals_el = Element::new("Normals".into());
        normals_el.tokens = vec!["0,0,1,0,0,1,0,0,1".to_string()];
        normals_el.parent_index = Some(root_idx);
        let normals_idx = arena.insert(normals_el);
        arena.get_mut(root_idx).unwrap().children.push(normals_idx);
        attrs.insert(
            "LayerElementNormal".into(),
            ElementAttribute::SubTree(Box::new(SubTreeAttribute {
                amphitheatre: arena,
                root_element_index: root_idx,
            })),
        );
        let mesh = MeshGeometry::try_from(owned_mesh(attrs)).unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.normals.len(), 3);
    }

    #[test]
    fn normals_by_polygon_vertex_direct() {
        let mut attrs = minimal_base_attrs();
        attrs.insert(
            "LayerElementNormal".into(),
            layer_element_normal("ByPolygonVertex", "Direct", "0,0,1,0,0,1,0,0,1"),
        );
        let mesh = MeshGeometry::try_from(owned_mesh(attrs)).unwrap();
        assert_eq!(
            mesh.normals,
            vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],]
        );
    }

    #[test]
    fn materials_all_same_replicates_first_index() {
        let mut attrs = minimal_base_attrs();
        attrs.insert(
            "LayerElementMaterial".into(),
            layer_element_material("AllSame", "IndexToDirect", "5"),
        );
        let mesh = MeshGeometry::try_from(owned_mesh(attrs)).unwrap();
        assert_eq!(mesh.material_indices, vec![5, 5, 5]);
    }

    #[test]
    fn wrong_object_kind_line_geometry() {
        let o = OwnedObject {
            object_index: 1,
            name: "G".into(),
            type_name: GEOMETRY_TYPE_NAME.into(),
            class_name: GEOMETRY_LINE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let err = MeshGeometry::try_from(o).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::WrongObjectKind { ref expected, .. } if expected == "MeshGeometry"
        ));
    }

    #[test]
    fn missing_mapping_information_type() {
        let mut attrs = minimal_base_attrs();
        let mut arena = ElementAmphitheatre::new();
        let root_idx = arena.insert(Element::new("LayerElementNormal".into()));
        append_layer_string_child(
            &mut arena,
            root_idx,
            super::ATTR_REFERENCE_INFORMATION_TYPE,
            "Direct",
        );
        let mut normals_el = Element::new("Normals".into());
        normals_el.tokens = vec!["0,0,1,0,0,1,0,0,1".to_string()];
        normals_el.parent_index = Some(root_idx);
        let normals_idx = arena.insert(normals_el);
        arena.get_mut(root_idx).unwrap().children.push(normals_idx);
        attrs.insert(
            "LayerElementNormal".into(),
            ElementAttribute::SubTree(Box::new(SubTreeAttribute {
                amphitheatre: arena,
                root_element_index: root_idx,
            })),
        );
        let err = MeshGeometry::try_from(owned_mesh(attrs)).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::MissingAttribute { ref name } if name == "MappingInformationType"
        ));
    }

    #[test]
    fn missing_reference_information_type() {
        let mut attrs = minimal_base_attrs();
        let mut arena = ElementAmphitheatre::new();
        let root_idx = arena.insert(Element::new("LayerElementNormal".into()));
        append_layer_string_child(
            &mut arena,
            root_idx,
            super::ATTR_MAPPING_INFORMATION_TYPE,
            "ByPolygonVertex",
        );
        let mut normals_el = Element::new("Normals".into());
        normals_el.tokens = vec!["0,0,1,0,0,1,0,0,1".to_string()];
        normals_el.parent_index = Some(root_idx);
        let normals_idx = arena.insert(normals_el);
        arena.get_mut(root_idx).unwrap().children.push(normals_idx);
        attrs.insert(
            "LayerElementNormal".into(),
            ElementAttribute::SubTree(Box::new(SubTreeAttribute {
                amphitheatre: arena,
                root_element_index: root_idx,
            })),
        );
        let err = MeshGeometry::try_from(owned_mesh(attrs)).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::MissingAttribute { ref name } if name == "ReferenceInformationType"
        ));
    }
}
