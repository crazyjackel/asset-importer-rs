//! FBX `AnimationCurve` — Assimp [`AnimationCurve`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXAnimation.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::OwnedObject;
use crate::Property;

use super::{AttrExtractor, FbxObjectTag, FbxTryFromReason, FbxTypeMismatch, fbx_object_tag};

const ATTR_KEY_TIME: &str = "KeyTime";
const ATTR_KEY_VALUE_FLOAT: &str = "KeyValueFloat";
const ATTR_KEY_ATTR_DATA_FLOAT: &str = "KeyAttrDataFloat";
const ATTR_KEY_ATTR_FLAGS: &str = "KeyAttrFlags";

#[derive(Debug, PartialEq)]
pub struct AnimationCurve {
    object: OwnedObject,
    pub keys: Vec<i64>,
    pub values: Vec<f32>,
    pub attributes: Vec<f32>,
    pub flags: Vec<u32>,
}

impl AnimationCurve {
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
}

impl TryFrom<OwnedObject> for AnimationCurve {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        // Check if tagged as AnimationCurve
        if fbx_object_tag(&o) != FbxObjectTag::AnimationCurve {
            return Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "AnimationCurve".to_string(),
            ));
        }

        let attrs = &o.attributes;

        // Extract KeyTime in keys
        let key_time_el = match attrs.extract_case_insensitive(ATTR_KEY_TIME) {
            Some(a) => a,
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: ATTR_KEY_TIME.to_string(),
                    },
                ));
            }
        };
        let keys: Vec<i64> = key_time_el
            .get_tokens()
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<i64>().ok())
            .collect();

        // Extract KeyValueFloat in values
        let key_value_el = match attrs.extract_case_insensitive(ATTR_KEY_VALUE_FLOAT) {
            Some(a) => a,
            None => {
                return Err(FbxTypeMismatch::new(
                    o,
                    FbxTryFromReason::MissingAttribute {
                        name: ATTR_KEY_VALUE_FLOAT.to_string(),
                    },
                ));
            }
        };
        let values: Vec<f32> = key_value_el
            .get_tokens()
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<f32>().ok())
            .collect();

        // Check if the number of keys and values match
        if keys.len() != values.len() {
            return Err(FbxTypeMismatch::new(
                o,
                FbxTryFromReason::InvalidAttributeFormat {
                    name: ATTR_KEY_VALUE_FLOAT.to_string(),
                    detail: format!(
                        "the number of key times ({}) does not match the number of keyframe values ({})",
                        keys.len(),
                        values.len()
                    ),
                },
            ));
        }

        // Check if the keys are in ascending order
        let mut is_sorted = true;
        for i in 0..keys.len().saturating_sub(1) {
            if keys[i] >= keys[i + 1] {
                is_sorted = false;
                break;
            }
        }
        if !is_sorted {
            return Err(FbxTypeMismatch::new(
                o,
                FbxTryFromReason::InvalidAttributeFormat {
                    name: ATTR_KEY_TIME.to_string(),
                    detail: "the keyframes are not in ascending order".to_string(),
                },
            ));
        }

        // Extract KeyAttrDataFloat in attributes
        let attributes = attrs
            .extract_case_insensitive(ATTR_KEY_ATTR_DATA_FLOAT)
            .map(|a| a.get_tokens())
            .unwrap_or(&Vec::new())
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<f32>().ok())
            .collect();

        // Extract KeyAttrFlags in flags
        let flags = attrs
            .extract_case_insensitive(ATTR_KEY_ATTR_FLAGS)
            .map(|a| a.get_tokens())
            .unwrap_or(&Vec::new())
            .iter()
            .flat_map(|t| t.split(','))
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse::<u32>().ok())
            .collect();

        Ok(AnimationCurve {
            object: o,
            keys,
            values,
            attributes,
            flags,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::objects::{ANIMATION_CURVE_CLASS_NAME, ANIMATION_CURVE_TYPE_NAME, FbxTryFromReason};
    use crate::{OwnedObject, Property};

    use super::{ATTR_KEY_TIME, ATTR_KEY_VALUE_FLOAT, AnimationCurve};

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    fn owned_curve(attrs: HashMap<String, ElementAttribute>) -> OwnedObject {
        OwnedObject {
            object_index: 1,
            name: "AnimCurve::X".into(),
            type_name: ANIMATION_CURVE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        }
    }

    #[test]
    fn parses_keys_values_optional_attr_flags() {
        let mut props = HashMap::new();
        props.insert("UserData".to_string(), Property::String("x".into()));

        let mut attrs = HashMap::new();
        attrs.insert(ATTR_KEY_TIME.into(), leaf(&["0,46186158000"]));
        attrs.insert(ATTR_KEY_VALUE_FLOAT.into(), leaf(&["0,1"]));
        attrs.insert("KeyAttrDataFloat".into(), leaf(&["0.5"]));
        attrs.insert("KeyAttrFlags".into(), leaf(&["1,2"]));

        let mut o = owned_curve(attrs);
        o.properties = props;

        let c = AnimationCurve::try_from(o).unwrap();
        assert_eq!(c.keys, vec![0i64, 46186158000]);
        assert_eq!(c.values, vec![0.0f32, 1.0]);
        assert_eq!(c.attributes, vec![0.5f32]);
        assert_eq!(c.flags, vec![1u32, 2u32]);
        assert_eq!(c.property("UserData"), Some(&Property::String("x".into())));
    }

    #[test]
    fn rejects_mismatched_key_value_lengths() {
        let mut attrs = HashMap::new();
        attrs.insert(ATTR_KEY_TIME.into(), leaf(&["0,1"]));
        attrs.insert(ATTR_KEY_VALUE_FLOAT.into(), leaf(&["0"]));
        let o = owned_curve(attrs);
        let err = AnimationCurve::try_from(o).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::InvalidAttributeFormat { ref name, .. }
            if name == ATTR_KEY_VALUE_FLOAT
        ));
    }

    #[test]
    fn rejects_non_ascending_keys() {
        let mut attrs = HashMap::new();
        attrs.insert(ATTR_KEY_TIME.into(), leaf(&["10,5"]));
        attrs.insert(ATTR_KEY_VALUE_FLOAT.into(), leaf(&["0,1"]));
        let o = owned_curve(attrs);
        let err = AnimationCurve::try_from(o).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::InvalidAttributeFormat { ref name, .. }
            if name == ATTR_KEY_TIME
        ));
    }
}
