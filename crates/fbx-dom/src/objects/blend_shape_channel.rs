//! FBX `Deformer` / `BlendShapeChannel` — Assimp [`BlendShapeChannel`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{AttrExtractor, FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

const ATTR_DEFORM_PERCENT: &str = "DeformPercent";
const ATTR_FULL_WEIGHTS: &str = "FullWeights";

#[derive(Debug, PartialEq)]
pub struct BlendShapeChannel {
    object: OwnedObject,
    pub percent: f32,
    pub full_weights: Vec<f32>,
}

impl BlendShapeChannel {
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

    pub fn deform_percent(&self) -> f32 {
        self.percent
    }

    pub fn full_weights(&self) -> &[f32] {
        &self.full_weights
    }
}

impl TryFrom<OwnedObject> for BlendShapeChannel {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::BlendShapeChannel) => {
                let percent = o
                    .attributes
                    .extract_case_insensitive(ATTR_DEFORM_PERCENT)
                    .and_then(|a| a.get_tokens().first())
                    .and_then(|t| t.trim().parse::<f32>().ok())
                    .unwrap_or(0.0);
                let full_weights = o
                    .attributes
                    .extract_case_insensitive(ATTR_FULL_WEIGHTS)
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
                Ok(BlendShapeChannel {
                    object: o,
                    percent,
                    full_weights,
                })
            }
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "BlendShapeChannel".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::objects::{DEFORMER_BLEND_SHAPE_CHANNEL_CLASS_NAME, DEFORMER_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::{ATTR_DEFORM_PERCENT, ATTR_FULL_WEIGHTS, BlendShapeChannel};

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    #[test]
    fn parses_percent_and_full_weights() {
        let mut attrs = HashMap::new();
        attrs.insert(ATTR_DEFORM_PERCENT.into(), leaf(&["37.5"]));
        attrs.insert(ATTR_FULL_WEIGHTS.into(), leaf(&["1,0.5,0.25"]));
        let mut props = HashMap::new();
        props.insert("Foo".into(), Property::String("bar".into()));
        let o = OwnedObject {
            object_index: 14,
            name: "BlendShapeChannel::A".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_BLEND_SHAPE_CHANNEL_CLASS_NAME.into(),
            properties: props,
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let c = BlendShapeChannel::try_from(o).unwrap();
        assert_eq!(c.deform_percent(), 37.5);
        assert_eq!(c.full_weights(), &[1.0, 0.5, 0.25]);
        assert_eq!(c.property("Foo"), Some(&Property::String("bar".into())));
    }
}
