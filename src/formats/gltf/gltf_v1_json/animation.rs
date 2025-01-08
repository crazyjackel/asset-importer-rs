use std::{collections::BTreeMap, fmt};

use super::{accessor::Accessor, root::StringIndex, validation::Checked};
use serde_derive::{Serialize, Deserialize};
use serde::{de,ser};

#[derive(Clone, Debug, PartialEq, Eq)]
enum SamplerInterpolation{
    Linear
}

impl ser::Serialize for SamplerInterpolation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(match *self {
            SamplerInterpolation::Linear => "LINEAR",
        })
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
                write!(f, "any of: {:?}", "LINEAR")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::SamplerInterpolation::*;
                Ok(match value {
                    "LINEAR" => Checked::Valid(Linear),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnimationChannelTarget{
    id: StringIndex<String>,
    path: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnimationChannel{
    sampler: String,
    target: AnimationChannelTarget
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnimationSampler{
    input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpolation: Option<Checked<SamplerInterpolation>>,
    output: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Animation{
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<Vec<AnimationChannel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<BTreeMap<String, StringIndex<Accessor>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    samplers: Option<BTreeMap<String, AnimationSampler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}

#[test]
fn test_animation_deserialize(){
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
    assert_eq!(Some("user-defined animation name".to_string()), animation_unwrap.name); 
}