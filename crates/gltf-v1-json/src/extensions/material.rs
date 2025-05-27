use std::fmt;

use gltf_v1_derive::Validate;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use crate::validation::Checked;

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub enum Technique {
    Blinn,
    Phong,
    Lambert,
    #[default]
    Constant,
}

impl Technique {
    pub const VALID_TECHNIQUES: &[&str] = &["LAMBERT", "PHONG", "BLINN", "CONSTANT"];
}

impl TryFrom<&str> for Technique {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "LAMBERT" => Ok(Technique::Lambert),
            "PHONG" => Ok(Technique::Phong),
            "BLINN" => Ok(Technique::Blinn),
            "CONSTANT" => Ok(Technique::Constant),
            _ => Err(()),
        }
    }
}

impl From<Technique> for &str {
    fn from(value: Technique) -> Self {
        match value {
            Technique::Lambert => "LAMBERT",
            Technique::Blinn => "BLINN",
            Technique::Phong => "PHONG",
            Technique::Constant => "CONSTANT",
        }
    }
}

impl ser::Serialize for Technique {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(Into::into(*self))
    }
}
impl<'de> de::Deserialize<'de> for Checked<Technique> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl de::Visitor<'_> for Visitor {
            type Value = Checked<Technique>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", Technique::VALID_TECHNIQUES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TryInto::try_into(value)
                    .map(Checked::Valid)
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

fn f32vec4_is_default(value: &[f32; 4]) -> bool {
    value[0] == 0.0 && value[1] == 0.0 && value[2] == 0.0 && value[3] == 1.0
}

fn default_f32vec4() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn f320_default() -> f32 {
    0.0
}
fn f321_default() -> f32 {
    1.0
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct MaterialValues {
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub ambient: [f32; 4],
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub diffuse: [f32; 4],
    #[serde(rename = "doubleSided")]
    pub double_sided: bool,
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub emission: [f32; 4],
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub specular: [f32; 4],
    #[serde(default = "f320_default")]
    pub shininess: f32,
    #[serde(default = "f321_default")]
    pub transparency: f32,
    pub transparent: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct MaterialCommon {
    pub technique: Checked<Technique>,
    pub values: MaterialValues,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Material {
    #[cfg(feature = "KHR_materials_common")]
    #[serde(rename = "KHR_materials_common")]
    pub khr_material_common: Option<MaterialCommon>,
    #[cfg(feature = "extensions")]
    #[serde(default, flatten)]
    pub others: Map<String, Value>,
}
