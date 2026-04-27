//! FBX `Deformer` / `Cluster` — Assimp [`Cluster`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedDocument, OwnedObject, Property};

use super::{AttrExtractor, FbxObjectTag, FbxTryFromReason, FbxTypeMismatch, Model, fbx_object_tag};

const ATTR_INDEXES: &str = "Indexes";
const ATTR_WEIGHTS: &str = "Weights";
const ATTR_TRANSFORM: &str = "Transform";
const ATTR_TRANSFORM_LINK: &str = "TransformLink";

#[derive(Debug, PartialEq)]
pub struct Cluster {
    object: OwnedObject,
    pub indices: Vec<u32>,
    pub weights: Vec<f32>,
    pub transform: [[f32; 4]; 4],
    pub transform_link: [[f32; 4]; 4],
}

impl Cluster {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }

    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.object.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.object.properties.get(name)
    }

    /// Resolve `Model -> Cluster` link (Assimp destination-side lookup).
    pub fn get_target_model<'a>(&'a self, document: &'a OwnedDocument) -> Option<&'a Model> {
        let cluster_id = self.inner().object_index;
        document
            .models
            .iter()
            .find(|model| model.inner().connected_object_ids.contains(&cluster_id))
    }
}

impl TryFrom<OwnedObject> for Cluster {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Cluster) {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Cluster".to_string()));
        }

        let attrs = &o.attributes;
        let transform = match attrs.extract_case_insensitive(ATTR_TRANSFORM) {
            Some(a) => match parse_matrix_4x4(a) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            },
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: ATTR_TRANSFORM.to_string(),
                    },
                ));
            }
        };
        let transform_link = match attrs.extract_case_insensitive(ATTR_TRANSFORM_LINK) {
            Some(a) => match parse_matrix_4x4(a) {
                Ok(m) => m,
                Err(reason) => return Err(FbxTypeMismatch::new(o, reason)),
            },
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: ATTR_TRANSFORM_LINK.to_string(),
                    },
                ));
            }
        };

        let indexes_attr = attrs.extract_case_insensitive(ATTR_INDEXES);
        let weights_attr = attrs.extract_case_insensitive(ATTR_WEIGHTS);
        if indexes_attr.is_some() != weights_attr.is_some() {
            return Err(FbxTypeMismatch::new(
                o,
                FbxTryFromReason::InvalidAttributeFormat {
                    name: "Cluster".to_string(),
                    detail: "either Indexes or Weights are missing from Cluster".to_string(),
                },
            ));
        }

        let indices = indexes_attr
            .map(|attr| {
                attr.get_tokens()
                    .iter()
                    .flat_map(|t| t.split(','))
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty())
                    .filter_map(|t| t.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .unwrap_or_default();
        let weights = weights_attr
            .map(|attr| {
                attr.get_tokens()
                    .iter()
                    .flat_map(|t| t.split(','))
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty())
                    .filter_map(|t| t.parse::<f32>().ok())
                    .collect::<Vec<f32>>()
            })
            .unwrap_or_default();
        if indices.len() != weights.len() {
            return Err(FbxTypeMismatch::new(
                o,
                FbxTryFromReason::InvalidAttributeFormat {
                    name: "Cluster".to_string(),
                    detail: "sizes of index and weight array don't match up".to_string(),
                },
            ));
        }

        Ok(Cluster {
            object: o,
            indices,
            weights,
            transform,
            transform_link,
        })
    }
}

fn parse_matrix_4x4(attr: &fbxscii::ElementAttribute) -> Result<[[f32; 4]; 4], FbxTryFromReason> {
    let flat: Vec<f32> = attr
        .get_tokens()
        .iter()
        .flat_map(|t| t.split(','))
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .filter_map(|t| t.parse::<f32>().ok())
        .collect();
    if flat.len() != 16 {
        return Err(FbxTryFromReason::InvalidAttributeFormat {
            name: "Matrix".to_string(),
            detail: format!("expected 16 floats, got {}", flat.len()),
        });
    }
    Ok([
        [flat[0], flat[1], flat[2], flat[3]],
        [flat[4], flat[5], flat[6], flat[7]],
        [flat[8], flat[9], flat[10], flat[11]],
        [flat[12], flat[13], flat[14], flat[15]],
    ])
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::objects::{
        DEFORMER_CLUSTER_CLASS_NAME, DEFORMER_TYPE_NAME, FbxTryFromReason, MODEL_TYPE_NAME, Model,
    };
    use crate::{OwnedObject, Property};
    use crate::OwnedDocument;

    use super::{ATTR_INDEXES, ATTR_TRANSFORM, ATTR_TRANSFORM_LINK, ATTR_WEIGHTS, Cluster};

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    fn matrix_csv() -> &'static str {
        "1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1"
    }

    #[test]
    fn parses_cluster_fields_and_properties() {
        let mut attrs = HashMap::new();
        attrs.insert(ATTR_TRANSFORM.into(), leaf(&[matrix_csv()]));
        attrs.insert(ATTR_TRANSFORM_LINK.into(), leaf(&[matrix_csv()]));
        attrs.insert(ATTR_INDEXES.into(), leaf(&["0,2,4"]));
        attrs.insert(ATTR_WEIGHTS.into(), leaf(&["0.2,0.5,1.0"]));
        let mut props = HashMap::new();
        props.insert("Foo".into(), Property::Bool(true));
        let o = OwnedObject {
            object_index: 12,
            name: "Cluster::A".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_CLUSTER_CLASS_NAME.into(),
            properties: props,
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let c = Cluster::try_from(o).unwrap();
        assert_eq!(c.indices, vec![0, 2, 4]);
        assert_eq!(c.weights, vec![0.2, 0.5, 1.0]);
        assert_eq!(c.transform[0], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(c.transform_link[3], [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(c.property("Foo"), Some(&Property::Bool(true)));
    }

    #[test]
    fn errors_when_only_one_of_indexes_weights_exists() {
        let mut attrs = HashMap::new();
        attrs.insert(ATTR_TRANSFORM.into(), leaf(&[matrix_csv()]));
        attrs.insert(ATTR_TRANSFORM_LINK.into(), leaf(&[matrix_csv()]));
        attrs.insert(ATTR_INDEXES.into(), leaf(&["0,1"]));
        let o = OwnedObject {
            object_index: 13,
            name: "Cluster::B".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_CLUSTER_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let err = Cluster::try_from(o).unwrap_err();
        assert!(matches!(err.reason, FbxTryFromReason::InvalidAttributeFormat { .. }));
    }

    #[test]
    fn resolves_target_model_from_connections() {
        let cluster = Cluster::try_from(OwnedObject {
            object_index: 20,
            name: "Cluster::T".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_CLUSTER_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::from([
                ("Transform".to_string(), leaf(&[matrix_csv()])),
                ("TransformLink".to_string(), leaf(&[matrix_csv()])),
            ]),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let model = Model::try_from(OwnedObject {
            object_index: 30,
            name: "Model::Node".into(),
            type_name: MODEL_TYPE_NAME.into(),
            class_name: "Mesh".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![20],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut owned = OwnedDocument::default();
        owned.models = vec![model];
        assert_eq!(
            cluster.get_target_model(&owned).map(|m| m.inner().object_index),
            Some(30)
        );
    }
}
