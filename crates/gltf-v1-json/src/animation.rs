use std::{collections::BTreeMap, fmt};

use crate::{texture::Sampler, Node};

use super::{accessor::Accessor, common::StringIndex, validation::Checked};
use gltf_v1_derive::Validate;
use indexmap::IndexMap;
use serde::{de, ser, Serialize};
use serde_json::value::Index;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum SamplerInterpolation {
    Linear,
}

impl SamplerInterpolation {
    pub const VALID_INTERPOLATION_TYPES: &[&str] = &["LINEAR"];
}

impl TryFrom<&str> for SamplerInterpolation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "LINEAR" => Ok(SamplerInterpolation::Linear),
            _ => Err(()),
        }
    }
}

impl From<SamplerInterpolation> for &str {
    fn from(value: SamplerInterpolation) -> Self {
        match value {
            SamplerInterpolation::Linear => "LINEAR",
        }
    }
}

impl ser::Serialize for SamplerInterpolation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(Into::into(*self))
    }
}
impl<'de> de::Deserialize<'de> for Checked<SamplerInterpolation> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<SamplerInterpolation>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "any of: {:?}",
                    SamplerInterpolation::VALID_INTERPOLATION_TYPES
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TryInto::try_into(value)
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum AnimationPath {
    Translation,
    Rotation,
    Scale,
}

impl AnimationPath {
    pub const VALID_INTERPOLATION_TYPES: &[&str] = &["translation", "rotation", "scale"];
}

impl TryFrom<&str> for AnimationPath {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "translation" => Ok(AnimationPath::Translation),
            "rotation" => Ok(AnimationPath::Rotation),
            "scale" => Ok(AnimationPath::Scale),
            _ => Err(()),
        }
    }
}

impl From<AnimationPath> for &str {
    fn from(value: AnimationPath) -> Self {
        match value {
            AnimationPath::Translation => "translation",
            AnimationPath::Rotation => "rotation",
            AnimationPath::Scale => "scale",
        }
    }
}

impl Serialize for AnimationPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str((*self).into())
    }
}

impl<'de> de::Deserialize<'de> for Checked<AnimationPath> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<AnimationPath>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", AnimationPath::VALID_INTERPOLATION_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct AnimationChannelTarget {
    pub id: StringIndex<Node>,
    pub path: Checked<AnimationPath>,
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct AnimationChannel {
    pub sampler: StringIndex<Sampler>,
    pub target: AnimationChannelTarget,
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct AnimationSampler {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Checked<SamplerInterpolation>>,
    pub output: String,
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Animation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<AnimationChannel>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<String, StringIndex<Accessor>>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub samplers: IndexMap<String, AnimationSampler>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[test]
fn test_animation_deserialize() {
    let data = r#"{
            "channels": [
                {
                    "sampler": "a_sampler",
                    "target": {
                        "id": "node_id",
                        "path": "rotation",
                        "extensions" : {
                            "extension_name" : {
                                "extension specific" : "value"
                            }
                        },
                        "extras" : {
                            "Application specific" : "The extra object can contain any properties."
                        }     
                    },
                     "extensions" : {
                         "extension_name" : {
                             "extension specific" : "value"
                         }
                     },
                    "extras" : {
                        "Application specific" : "The extra object can contain any properties."
                    }     
                }
            ],
            "name": "user-defined animation name",
            "parameters": {
                "TIME": "time_accessor",
                "rotation": "rotation_accessor"
            },
            "samplers": {
                "a_sampler": {
                    "input": "TIME",
                    "interpolation": "LINEAR",
                    "output": "rotation",
		            "extensions" : {
		               "extension_name" : {
		                  "extension specific" : "value"
		               }
		            },
                    "extras" : {
                        "Application specific" : "The extra object can contain any properties."
                    }     
                }
            },
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let animation: Result<Animation, _> = serde_json::from_str(data);
    let animation_unwrap = animation.unwrap();
    println!("{}", serde_json::to_string(&animation_unwrap).unwrap());
    assert_eq!(
        Some("user-defined animation name".to_string()),
        animation_unwrap.name
    );
}
