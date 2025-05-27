use std::fmt;

use gltf_v1_derive::Validate;
use map::IndexMap;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use crate::{gltf::Get, validation::Checked, Path, Root, StringIndex};

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub enum Type {
    #[default]
    Ambient,
    Directional,
    Point,
    Spot,
}

impl Type {
    pub const VALID_TYPES: &[&str] = &["ambient", "directional", "point", "spot"];
}
impl TryFrom<&str> for Type {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ambient" => Ok(Type::Ambient),
            "directional" => Ok(Type::Directional),
            "point" => Ok(Type::Point),
            "spot" => Ok(Type::Spot),
            _ => Err(()),
        }
    }
}

impl From<Type> for &str {
    fn from(value: Type) -> Self {
        match value {
            Type::Ambient => "ambient",
            Type::Directional => "directional",
            Type::Point => "point",
            Type::Spot => "spot",
        }
    }
}

impl ser::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(Into::into(*self))
    }
}

impl<'de> de::Deserialize<'de> for Checked<Type> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Type>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", Type::VALID_TYPES)
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
fn half_pi_default() -> f32 {
    std::f32::consts::PI / 2.0
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct DirectionalLight {
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub color: [f32; 4],
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AmbientLight {
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub color: [f32; 4],
}
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PointLight {
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub color: [f32; 4],
    #[serde(rename = "constantAttenuation", default = "f320_default")]
    pub constant_attenuation: f32,
    #[serde(rename = "linearAttenuation", default = "f321_default")]
    pub linear_attenuation: f32,
    #[serde(rename = "quadraticAttenuation", default = "f321_default")]
    pub quadratic_attenuation: f32,
    #[serde(default = "f320_default")]
    pub distance: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SpotLight {
    #[serde(
        skip_serializing_if = "f32vec4_is_default",
        default = "default_f32vec4"
    )]
    pub color: [f32; 4],
    #[serde(rename = "constantAttenuation", default = "f320_default")]
    pub constant_attenuation: f32,
    #[serde(rename = "linearAttenuation", default = "f321_default")]
    pub linear_attenuation: f32,
    #[serde(rename = "quadraticAttenuation", default = "f321_default")]
    pub quadratic_attenuation: f32,
    #[serde(default = "f320_default")]
    pub distance: f32,
    #[serde(rename = "falloffAngle", default = "half_pi_default")]
    pub falloff_angle: f32,
    #[serde(rename = "falloffExponent", default = "f320_default")]
    pub falloff_exponent: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[gltf(validate_hook = "light_validate_light")]
pub struct Light {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<AmbientLight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directional: Option<DirectionalLight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointLight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<SpotLight>,
    #[serde(rename = "type")]
    pub type_: Checked<Type>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Lights {
    #[serde(default)]
    #[serde(rename = "lights")]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub lights: IndexMap<String, Light>,
}

#[cfg(feature = "KHR_materials_common")]
impl Get<Light> for Root {
    fn get(&self, index: StringIndex<Light>) -> Option<&Light> {
        self.extensions
            .as_ref()?
            .ktr_materials_common
            .as_ref()?
            .lights
            .get(index.value())
    }
}

fn light_validate_light<P, R>(light: &Light, _root: &Root, path: P, report: &mut R)
where
    P: Fn() -> Path,
    R: FnMut(&dyn Fn() -> Path, crate::validation::Error),
{
    match light.type_ {
        Checked::Valid(Type::Spot) if light.spot.is_none() => {
            report(&path, crate::validation::Error::Missing)
        }
        Checked::Valid(Type::Ambient) if light.ambient.is_none() => {
            report(&path, crate::validation::Error::Missing)
        }
        Checked::Valid(Type::Point) if light.point.is_none() => {
            report(&path, crate::validation::Error::Missing)
        }
        Checked::Valid(Type::Directional) if light.directional.is_none() => {
            report(&path, crate::validation::Error::Missing)
        }
        _ => {}
    }
}
