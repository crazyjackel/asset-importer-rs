use std::{collections::BTreeMap, fmt};

use serde::{de, Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

use super::{node::Node, root::StringIndex, validation::Checked};

pub const BYTE: u32 = 5120;
pub const UNSIGNED_BYTE: u32 = 5121;
pub const SHORT: u32 = 5122;
pub const UNSIGNED_SHORT: u32 = 5123;
pub const INT: u32 = 5124;
pub const UNSIGNED_INT: u32 = 5125;
pub const FLOAT: u32 = 5126;
pub const FLOAT_VEC2: u32 = 35664;
pub const FLOAT_VEC3: u32 = 35665;
pub const FLOAT_VEC4: u32 = 35666;
pub const INT_VEC2: u32 = 35667;
pub const INT_VEC3: u32 = 35668;
pub const INT_VEC4: u32 = 35669;
pub const BOOL: u32 = 35670;
pub const BOOL_VEC2: u32 = 35671;
pub const BOOL_VEC3: u32 = 35672;
pub const BOOL_VEC4: u32 = 35673;
pub const FLOAT_MAT2: u32 = 35674;
pub const FLOAT_MAT3: u32 = 35675;
pub const FLOAT_MAT4: u32 = 35676;
pub const SAMPLER_2D: u32 = 35678;

pub const VALID_PARAMETER_TYPES: &[u32] = &[
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    INT,
    UNSIGNED_INT,
    FLOAT,
    FLOAT_VEC2,
    FLOAT_VEC3,
    FLOAT_VEC4,
    INT_VEC2,
    INT_VEC3,
    INT_VEC4,
    BOOL,
    BOOL_VEC2,
    BOOL_VEC3,
    BOOL_VEC4,
    FLOAT_MAT2,
    FLOAT_MAT3,
    FLOAT_MAT4,
    SAMPLER_2D,
];

#[derive(Clone, Debug, Copy)]
pub enum ParameterType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    IntVec2,
    IntVec3,
    IntVec4,
    Bool,
    BoolVec2,
    BoolVec3,
    BoolVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    Sampler2d,
}
impl ParameterType {
    pub const fn to_repr(self) -> u32 {
        match self {
            ParameterType::Byte => BYTE,
            ParameterType::UnsignedByte => UNSIGNED_BYTE,
            ParameterType::Short => SHORT,
            ParameterType::UnsignedShort => UNSIGNED_SHORT,
            ParameterType::Int => INT,
            ParameterType::UnsignedInt => UNSIGNED_INT,
            ParameterType::Float => FLOAT,
            ParameterType::FloatVec2 => FLOAT_VEC2,
            ParameterType::FloatVec3 => FLOAT_VEC3,
            ParameterType::FloatVec4 => FLOAT_VEC4,
            ParameterType::IntVec2 => INT_VEC2,
            ParameterType::IntVec3 => INT_VEC3,
            ParameterType::IntVec4 => INT_VEC4,
            ParameterType::Bool => BOOL,
            ParameterType::BoolVec2 => BOOL_VEC2,
            ParameterType::BoolVec3 => BOOL_VEC3,
            ParameterType::BoolVec4 => BOOL_VEC4,
            ParameterType::FloatMat2 => FLOAT_MAT2,
            ParameterType::FloatMat3 => FLOAT_MAT3,
            ParameterType::FloatMat4 => FLOAT_MAT4,
            ParameterType::Sampler2d => SAMPLER_2D,
        }
    }
}

impl Serialize for ParameterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
    }
}

impl<'de> Deserialize<'de> for Checked<ParameterType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<ParameterType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_PARAMETER_TYPES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::ParameterType::*;
                Ok(match value as u32 {
                    BYTE => Checked::Valid(Byte),
                    UNSIGNED_BYTE => Checked::Valid(UnsignedByte),
                    SHORT => Checked::Valid(Short),
                    UNSIGNED_SHORT => Checked::Valid(UnsignedShort),
                    INT => Checked::Valid(Int),
                    UNSIGNED_INT => Checked::Valid(UnsignedInt),
                    FLOAT => Checked::Valid(Float),
                    FLOAT_VEC2 => Checked::Valid(FloatVec2),
                    FLOAT_VEC3 => Checked::Valid(FloatVec3),
                    FLOAT_VEC4 => Checked::Valid(FloatVec4),
                    INT_VEC2 => Checked::Valid(IntVec2),
                    INT_VEC3 => Checked::Valid(IntVec3),
                    INT_VEC4 => Checked::Valid(IntVec4),
                    BOOL => Checked::Valid(Bool),
                    BOOL_VEC2 => Checked::Valid(BoolVec2),
                    BOOL_VEC3 => Checked::Valid(BoolVec3),
                    BOOL_VEC4 => Checked::Valid(BoolVec4),
                    FLOAT_MAT2 => Checked::Valid(FloatMat2),
                    FLOAT_MAT3 => Checked::Valid(FloatMat3),
                    FLOAT_MAT4 => Checked::Valid(FloatMat4),
                    SAMPLER_2D => Checked::Valid(Sampler2d),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum ParameterValue {
    Number(f32),
    Boolean(bool),
    String(String),
    NumberArray(Vec<f32>),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
}

impl<'de> Deserialize<'de> for ParameterValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ParameterValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ParameterValueVisitor {
            type Value = ParameterValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number, boolean, string, or array of these types")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ParameterValue::Number(value as f32))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ParameterValue::Boolean(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ParameterValue::String(value.to_string()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Try to deserialize as Vec<f32>
                if let Some(first) = seq.next_element::<f64>()? {
                    let mut vec = vec![first as f32];
                    while let Some(val) = seq.next_element::<f64>()? {
                        vec.push(val as f32);
                    }
                    return Ok(ParameterValue::NumberArray(vec));
                }

                // Try to deserialize as Vec<bool>
                if let Some(first) = seq.next_element::<bool>()? {
                    let mut vec = vec![first];
                    while let Some(val) = seq.next_element()? {
                        vec.push(val);
                    }
                    return Ok(ParameterValue::BoolArray(vec));
                }

                // Try to deserialize as Vec<String>
                if let Some(first) = seq.next_element::<String>()? {
                    let mut vec = vec![first];
                    while let Some(val) = seq.next_element()? {
                        vec.push(val);
                    }
                    return Ok(ParameterValue::StringArray(vec));
                }

                Err(de::Error::custom(
                    "Expected an array of numbers, booleans, or strings",
                ))
            }
        }

        deserializer.deserialize_any(ParameterValueVisitor)
    }
}

pub const BLEND: u32 = 3042;
pub const CULL_FACE: u32 = 2884;
pub const DEPTH_TEST: u32 = 2929;
pub const POLYGON_OFFSET_FILL: u32 = 32823;
pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926;
pub const SCISSOR_TEST: u32 = 3089;

pub const VALID_WEB_GL_STATES: &[u32] = &[
    BLEND,
    CULL_FACE,
    DEPTH_TEST,
    POLYGON_OFFSET_FILL,
    SAMPLE_ALPHA_TO_COVERAGE,
    SCISSOR_TEST,
];

#[derive(Clone, Debug)]
pub enum WebGLState {
    Blend,
    CullFace,
    DepthTest,
    PolygonOffsetFill,
    SampleAlphaToCoverage,
    ScissorTest,
}

impl Serialize for WebGLState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            WebGLState::Blend => serializer.serialize_u32(BLEND),
            WebGLState::CullFace => serializer.serialize_u32(CULL_FACE),
            WebGLState::DepthTest => serializer.serialize_u32(DEPTH_TEST),
            WebGLState::PolygonOffsetFill => serializer.serialize_u32(POLYGON_OFFSET_FILL),
            WebGLState::SampleAlphaToCoverage => serializer.serialize_u32(SAMPLE_ALPHA_TO_COVERAGE),
            WebGLState::ScissorTest => serializer.serialize_u32(SCISSOR_TEST),
        }
    }
}

impl<'de> Deserialize<'de> for Checked<WebGLState> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<WebGLState>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_WEB_GL_STATES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::WebGLState::*;
                Ok(match value as u32 {
                    BLEND => Checked::Valid(Blend),
                    CULL_FACE => Checked::Valid(CullFace),
                    DEPTH_TEST => Checked::Valid(DepthTest),
                    POLYGON_OFFSET_FILL => Checked::Valid(PolygonOffsetFill),
                    SAMPLE_ALPHA_TO_COVERAGE => Checked::Valid(SampleAlphaToCoverage),
                    SCISSOR_TEST => Checked::Valid(ScissorTest),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u32(Visitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TechniqueParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<StringIndex<Node>>,
    #[serde(rename = "type")]
    parameter_type: Checked<ParameterType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ParameterValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TechniqueStateFunction {
    #[serde(rename = "blendColor", skip_serializing_if = "Option::is_none")]
    blend_color: Option<[f32; 4]>,
    #[serde(rename = "blendEquationSeparate", skip_serializing_if = "Option::is_none")]
    blend_equation_seperate: Option<[u32; 2]>,
    #[serde(rename = "blendFuncSeparate", skip_serializing_if = "Option::is_none")]
    blend_func_seperate: Option<[u32; 4]>,
    #[serde(rename = "colorMask", skip_serializing_if = "Option::is_none")]
    color_mask: Option<[bool; 4]>,
    #[serde(rename = "cullFace", skip_serializing_if = "Option::is_none")]
    cull_face: Option<[u32; 1]>,
    #[serde(rename = "depthFunc", skip_serializing_if = "Option::is_none")]
    depth_func: Option<[u32; 1]>,
    #[serde(rename = "depthMask", skip_serializing_if = "Option::is_none")]
    depth_mask: Option<[bool; 1]>,
    #[serde(rename = "depthRange", skip_serializing_if = "Option::is_none")]
    depth_range: Option<[f32; 2]>,
    #[serde(rename = "frontFace", skip_serializing_if = "Option::is_none")]
    front_face: Option<[u32; 1]>,
    #[serde(rename = "lineWidth", skip_serializing_if = "Option::is_none")]
    line_width: Option<[f32; 1]>,
    #[serde(rename = "polygonOffset", skip_serializing_if = "Option::is_none")]
    polygon_offset: Option<[f32; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scissor: Option<[f32; 4]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TechniqueState {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<Vec<Checked<WebGLState>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<TechniqueStateFunction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Technique {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<BTreeMap<String, TechniqueParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<BTreeMap<String, StringIndex<TechniqueParameter>>>,
    program: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniforms: Option<BTreeMap<String, StringIndex<TechniqueParameter>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    states: Option<TechniqueState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technique: Option<StringIndex<Technique>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<BTreeMap<String, ParameterValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[test]
fn test_technique_deserialize() {
    let data = r#"{
            "name": "user-defined technique name",
            "parameters": {
                "ambient": {
                    "type": 35666,
		            "extensions" : {
		               "extension_name" : {
		                  "extension specific" : "value"
		               }
		            },
                    "extras" : {
                        "Application specific" : "The extra object can contain any properties."
                    }
                },
                "diffuse": {
                    "type": 35678
                },
                "lightColor": {
                    "type": 35665,
                    "value": [
                        1,
                        1,
                        1
                    ]
                },
                "lightTransform": {
                    "node": "directional_light_node_id",
                    "type": 35676
                },
                "modelViewMatrix": {
                    "semantic": "MODELVIEW",
                    "type": 35676
                },
                "projectionMatrix": {
                    "semantic": "PROJECTION",
                    "type": 35676
                },
                "normalMatrix": {
                    "semantic": "MODELVIEWINVERSETRANSPOSE",
                    "type": 35675
                },

                "position": {
                    "semantic": "POSITION",
                    "type": 35665
                },
                "normal": {
                    "semantic": "NORMAL",
                    "type": 35665
                },
                "texcoord": {
                    "semantic": "TEXCOORD_0",
                    "type": 35664
                },

                "joint": {
                    "semantic": "JOINT",
                    "type": 35666
                },
                "jointMatrix": {
                    "semantic": "JOINTMATRIX",
                    "type": 35676
                },
                "weight": {
                    "semantic": "WEIGHT",
                    "type": 35666
                }
            },
            "attributes": {
                "a_position": "position",
                "a_normal": "normal",
                "a_texcoord0": "texcoord0",
                "a_joint": "joint",
                "a_weight": "weight"
            },
            "program": "program_id",
            "uniforms": {
                "u_ambient": "ambient",
                "u_diffuse": "diffuse",
                "u_lightColor": "lightColor",
                "u_lightTransformMatrix": "lightTransform",
                "u_modelViewMatrix": "modelViewMatrix",
                "u_projectionMatrix": "projectionMatrix",
                "u_normalMatrix": "normalMatrix",
                "u_jointMatrix": "jointMatrix"
            },
            "states" : {
                "enable" : [3042, 2884, 2929, 32823, 32926, 3089],
                "functions" : {
                    "blendColor": [0.0, 0.0, 0.0, 0.0],
                    "blendEquationSeparate" : [32774, 32774],
                    "blendFuncSeparate" : [1, 0, 1, 0],
                    "colorMask" : [true, true, true, true],
                    "cullFace" : [1029],
                    "depthFunc" : [513],
                    "depthMask" : [true],
                    "depthRange" : [0.0, 1.0],
                    "frontFace" : [2305],
                    "lineWidth" : [1.0],
                    "polygonOffset" : [0.0, 0.0],
                    "scissor" : [0, 0, 0, 0],
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
    let technique: Result<Technique, _> = serde_json::from_str(data);
    let technique_unwrap = technique.unwrap();
    println!("{}", serde_json::to_string(&technique_unwrap).unwrap());
    assert_eq!(
        Some("user-defined technique name".to_string()),
        technique_unwrap.name
    );
}
#[test]
fn test_material_deserialize() {
    let data = r#"{
            "technique": "technique_id",
            "values": {
                "ambient": [
                    0,
                    0,
                    0,
                    1
                ],
                "diffuse": "texture_image_0",
                "shininess": 38.4
            },
            "name": "user-defined material name",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let material: Result<Material, _> = serde_json::from_str(data);
    let material_unwrap = material.unwrap();
    println!("{}", serde_json::to_string(&material_unwrap).unwrap());
    assert_eq!(
        Some("user-defined material name".to_string()),
        material_unwrap.name
    );
}
