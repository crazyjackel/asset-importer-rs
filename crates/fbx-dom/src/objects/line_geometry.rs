//! FBX `Geometry` / `Line` — Assimp [`LineGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMeshGeometry.cpp).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::AttrExtractor;
use super::{FbxObjectTag, FbxTryFromReason, FbxTypeMismatch, fbx_object_tag};

#[derive(Debug, PartialEq)]
pub struct LineGeometry {
    pub object: OwnedObject,
    pub points: Vec<[f32; 3]>,
    pub point_indices: Vec<i32>,
}

impl LineGeometry {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

impl TryFrom<OwnedObject> for LineGeometry {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::LineGeometry => {}
            _ => {
                return Err(FbxTypeMismatch::wrong_object_kind(
                    o,
                    "LineGeometry".to_string(),
                ));
            }
        }

        let attrs = &o.attributes;

        let points_tokens = match attrs.extract_case_insensitive("Points") {
            Some(a) => a.get_tokens(),
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: "Points".to_string(),
                    },
                ));
            }
        };
        let points = points_tokens
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<f32>().ok())
            .collect::<Vec<f32>>()
            .chunks_exact(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect::<Vec<[f32; 3]>>();

        let idx_tokens = match attrs.extract_case_insensitive("PointsIndex") {
            Some(a) => a.get_tokens(),
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: "PointsIndex".to_string(),
                    },
                ));
            }
        };
        let point_indices = idx_tokens
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<i32>().ok())
            .collect();

        Ok(LineGeometry {
            object: o,
            points,
            point_indices,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, convert::TryFrom};

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::OwnedObject;

    use super::super::{GEOMETRY_LINE_CLASS_NAME, GEOMETRY_TYPE_NAME};
    use super::LineGeometry;

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    fn owned_line(attrs: HashMap<String, ElementAttribute>) -> OwnedObject {
        OwnedObject {
            object_index: 10,
            name: "Geometry::TestLine".into(),
            type_name: GEOMETRY_TYPE_NAME.into(),
            class_name: GEOMETRY_LINE_CLASS_NAME.into(),
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
        attrs.insert("Points".into(), leaf(&["0,0,0,1,0,0,0,1,0"]));
        attrs.insert("PointsIndex".into(), leaf(&["0,1,2"]));
        let o = owned_line(attrs);
        let lg = LineGeometry::try_from(o).unwrap();
        let expected_points = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let expected_point_indices = vec![0, 1, 2];
        assert_eq!(lg.points, expected_points);
        assert_eq!(lg.point_indices, expected_point_indices);
    }
}
