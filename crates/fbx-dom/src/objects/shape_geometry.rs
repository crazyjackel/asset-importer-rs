//! FBX `Geometry` / `Shape` — Assimp [`ShapeGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.cpp).

use std::convert::TryFrom;
use std::num::ParseFloatError;
use std::num::ParseIntError;

use crate::OwnedObject;

use super::AttrExtractor;
use super::{FbxObjectTag, FbxTryFromReason, FbxTypeMismatch, fbx_object_tag};

#[derive(Debug, PartialEq)]
pub struct ShapeGeometry {
    pub object: OwnedObject,
    pub indices: Vec<u32>,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
}

impl ShapeGeometry {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

impl TryFrom<OwnedObject> for ShapeGeometry {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::ShapeGeometry => {}
            _ => {
                return Err(FbxTypeMismatch::wrong_object_kind(
                    o,
                    "ShapeGeometry".to_string(),
                ));
            }
        }

        let attrs = &o.attributes;

        let idx_tokens = match attrs.extract_case_insensitive("Indexes") {
            Some(a) => a.get_tokens(),
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: "Indexes".to_string(),
                    },
                ));
            }
        };
        let indices_result = idx_tokens
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>();
        let Ok(indices) = indices_result else {
            return Err(FbxTypeMismatch::new(
                o,
                FbxTryFromReason::InvalidAttributeFormat {
                    name: "Indexes".to_string(),
                    detail: format!("invalid int token: {}", indices_result.unwrap_err()),
                },
            ));
        };

        let verts_tokens = match attrs.extract_case_insensitive("Vertices") {
            Some(a) => a.get_tokens(),
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: "Vertices".to_string(),
                    },
                ));
            }
        };
        let vertices_result = verts_tokens
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.parse::<f32>())
            .collect::<Result<Vec<f32>, ParseFloatError>>();
        let Ok(vertices_unchunked) = vertices_result else {
            return Err(FbxTypeMismatch::new(
                o,
                FbxTryFromReason::InvalidAttributeFormat {
                    name: "Vertices".to_string(),
                    detail: format!("invalid float token: {}", vertices_result.unwrap_err()),
                },
            ));
        };
        let vertices = vertices_unchunked
            .chunks_exact(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect::<Vec<[f32; 3]>>();

        // Validate that no index is out of bounds
        for index in indices.iter() {
            if *index >= vertices.len() as u32 {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::InvalidAttributeFormat {
                        name: "Indexes".to_string(),
                        detail: format!("index out of bounds: {}", *index),
                    },
                ));
            }
        }

        let normals = if let Some(n_attr) = attrs.extract_case_insensitive("Normals") {
            let n_tokens = n_attr.get_tokens();
            let normals_result = n_tokens
                .iter()
                .flat_map(|t| t.split(','))
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .map(|t| t.parse::<f32>())
                .collect::<Result<Vec<f32>, ParseFloatError>>()
                .unwrap_or_default(); // If the parse fails, return an empty vector. This is intentional.
            
            normals_result
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect::<Vec<[f32; 3]>>()
        } else {
            Vec::new()
        };

        Ok(ShapeGeometry {
            object: o,
            indices,
            vertices,
            normals,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::OwnedObject;

    use super::super::{GEOMETRY_SHAPE_CLASS_NAME, GEOMETRY_TYPE_NAME};
    use super::ShapeGeometry;

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    fn owned_shape(attrs: HashMap<String, ElementAttribute>) -> OwnedObject {
        OwnedObject {
            object_index: 10,
            name: "Geometry::TestShape".into(),
            type_name: GEOMETRY_TYPE_NAME.into(),
            class_name: GEOMETRY_SHAPE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        }
    }

    #[test]
    fn basic_parse() {
        let mut attrs = HashMap::new();
        attrs.insert("Indexes".into(), leaf(&["0,1,2"]));
        attrs.insert("Vertices".into(), leaf(&["0,0,0,1,0,0,0,1,0"]));
        let o = owned_shape(attrs);
        let sg = ShapeGeometry::try_from(o).unwrap();
        let expected_indices = vec![0, 1, 2];
        let expected_vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let expected_normals = Vec::<[f32; 3]>::new();
        assert_eq!(sg.indices, expected_indices);
        assert_eq!(sg.vertices, expected_vertices);
        assert_eq!(sg.normals, expected_normals);
    }
}
