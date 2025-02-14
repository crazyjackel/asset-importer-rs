use std::fmt;

use gltf_v1_derive::Validate;
use serde::de;
use serde::{Deserialize, Serialize};

use super::common::StringIndex;
use super::validation::Checked;

pub const FRAGMENT_SHADER: u32 = 35632;
pub const VERTEX_SHADER: u32 = 35633;

#[derive(Clone, Debug, Copy)]
pub enum ShaderType {
    FragmentShader,
    VertexShader,
}

impl ShaderType {
    pub const VALID_SHADER_TYPE: &[u32] = &[FRAGMENT_SHADER, VERTEX_SHADER];
}

impl From<ShaderType> for u32 {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::FragmentShader => FRAGMENT_SHADER,
            ShaderType::VertexShader => VERTEX_SHADER,
        }
    }
}

impl TryFrom<u32> for ShaderType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            FRAGMENT_SHADER => Ok(ShaderType::FragmentShader),
            VERTEX_SHADER => Ok(ShaderType::VertexShader),
            _ => Err(()),
        }
    }
}
impl Serialize for ShaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<ShaderType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<ShaderType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", ShaderType::VALID_SHADER_TYPE)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok((value as u32)
                    .try_into()
                    .map(|x| Checked::Valid(x))
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Shader {
    pub uri: String,
    #[serde(rename = "type")]
    pub type_: Checked<ShaderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Program {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<String>,
    #[serde(rename = "fragmentShader")]
    pub fragment_shader: StringIndex<Shader>,
    #[serde(rename = "vertexShader")]
    pub vertex_shader: StringIndex<Shader>,
    pub name: Option<String>,
}

#[test]
fn test_program_deserialize() {
    let data = r#"{
            "attributes": [
                "a_normal",
                "a_position"
            ],
            "fragmentShader": "fs_id",
            "name": "user-defined program name",
            "vertexShader": "vs_id",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let program: Result<Program, _> = serde_json::from_str(data);
    let program_unwrap = program.unwrap();
    println!("{}", serde_json::to_string(&program_unwrap).unwrap());
    assert_eq!(
        Some("user-defined program name".to_string()),
        program_unwrap.name
    );
}

#[test]
fn test_shader_deserialize() {
    let data = r#"{
            "name": "user-defined vertex shader name",
            "uri" : "vertexshader.glsl",
            "type": 35633,
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }
        }"#;
    let shader: Result<Shader, _> = serde_json::from_str(data);
    let shader_unwrap = shader.unwrap();
    println!("{}", serde_json::to_string(&shader_unwrap).unwrap());
    assert_eq!(
        Some("user-defined vertex shader name".to_string()),
        shader_unwrap.name
    );
}
