
use std::fmt;

use serde::de;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

use super::root::StringIndex;
use super::validation::Checked;

pub const FRAGMENT_SHADER: u32 = 35632;
pub const VERTEX_SHADER: u32 = 35633;
pub const VALID_SHADER_TYPE: &[u32] = &[FRAGMENT_SHADER, VERTEX_SHADER];

#[derive(Clone, Debug, Copy)]
pub enum ShaderType{
    FragmentShader,
    VertexShader
}
impl ShaderType {
    pub const fn to_repr(self) -> u32 {
        match self {
            ShaderType::FragmentShader => FRAGMENT_SHADER,
            ShaderType::VertexShader => VERTEX_SHADER,
        }
    }
}
impl Serialize for ShaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_SHADER_TYPE)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::ShaderType::*;
                Ok(match value as u32 {
                    FRAGMENT_SHADER => Checked::Valid(FragmentShader),
                    VERTEX_SHADER => Checked::Valid(VertexShader),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shader {
    uri: String,
    #[serde(rename = "type")]
    shader_type: Checked<ShaderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Program {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<String>>,
    #[serde(rename = "fragmentShader")]
    fragment_shader: StringIndex<Shader>,
    #[serde(rename = "vertexShader")]
    vertex_shader: StringIndex<Shader>,
    name: Option<String>
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
