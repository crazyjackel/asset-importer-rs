use std::fmt;

use gltf_v1_derive::Validate;
use indexmap::IndexMap;
use serde::{de, Deserialize, Serialize};

use crate::{
    validation::{Error, USize64},
    Path, Program, Root,
};

use super::{common::StringIndex, node::Node, validation::Checked};

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

#[derive(Clone, Debug, Copy, Default)]
pub enum ParameterType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    #[default]
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
}

impl TryFrom<u32> for ParameterType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            BYTE => Ok(ParameterType::Byte),
            UNSIGNED_BYTE => Ok(ParameterType::UnsignedByte),
            SHORT => Ok(ParameterType::Short),
            UNSIGNED_SHORT => Ok(ParameterType::UnsignedShort),
            INT => Ok(ParameterType::Int),
            UNSIGNED_INT => Ok(ParameterType::UnsignedInt),
            FLOAT => Ok(ParameterType::Float),
            FLOAT_VEC2 => Ok(ParameterType::FloatVec2),
            FLOAT_VEC3 => Ok(ParameterType::FloatVec3),
            FLOAT_VEC4 => Ok(ParameterType::FloatVec4),
            INT_VEC2 => Ok(ParameterType::IntVec2),
            INT_VEC3 => Ok(ParameterType::IntVec3),
            INT_VEC4 => Ok(ParameterType::IntVec4),
            BOOL => Ok(ParameterType::Bool),
            BOOL_VEC2 => Ok(ParameterType::BoolVec2),
            BOOL_VEC3 => Ok(ParameterType::BoolVec3),
            BOOL_VEC4 => Ok(ParameterType::BoolVec4),
            FLOAT_MAT2 => Ok(ParameterType::FloatMat2),
            FLOAT_MAT3 => Ok(ParameterType::FloatMat3),
            FLOAT_MAT4 => Ok(ParameterType::FloatMat4),
            SAMPLER_2D => Ok(ParameterType::Sampler2d),
            _ => Err(()),
        }
    }
}

impl From<ParameterType> for u32 {
    fn from(value: ParameterType) -> Self {
        match value {
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
        serializer.serialize_u32((*self).into())
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
                write!(f, "any of: {:?}", ParameterType::VALID_PARAMETER_TYPES)
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

#[derive(Clone, Debug, serde_derive::Serialize)]
pub enum ParameterValue {
    Number(f32),
    Boolean(bool),
    String(String),
    NumberArray(Vec<f32>),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
}

impl<'de> Deserialize<'de> for Checked<ParameterValue> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ParameterValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ParameterValueVisitor {
            type Value = Checked<ParameterValue>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number, boolean, string, or array of these types")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Checked::Valid(ParameterValue::Number(value as f32)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Checked::Valid(ParameterValue::Number(value as f32)))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Checked::Valid(ParameterValue::Number(value as f32)))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Checked::Valid(ParameterValue::Boolean(value)))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Checked::Valid(ParameterValue::String(value.to_string())))
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
                    return Ok(Checked::Valid(ParameterValue::NumberArray(vec)));
                }

                // Try to deserialize as Vec<bool>
                if let Some(first) = seq.next_element::<bool>()? {
                    let mut vec = vec![first];
                    while let Some(val) = seq.next_element()? {
                        vec.push(val);
                    }
                    return Ok(Checked::Valid(ParameterValue::BoolArray(vec)));
                }

                // Try to deserialize as Vec<String>
                if let Some(first) = seq.next_element::<String>()? {
                    let mut vec = vec![first];
                    while let Some(val) = seq.next_element()? {
                        vec.push(val);
                    }
                    return Ok(Checked::Valid(ParameterValue::StringArray(vec)));
                }

                Ok(Checked::Invalid)
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

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum WebGLState {
    Blend,
    CullFace,
    DepthTest,
    PolygonOffsetFill,
    SampleAlphaToCoverage,
    ScissorTest,
}

impl WebGLState {
    pub const VALID_WEB_GL_STATES: &[u32] = &[
        BLEND,
        CULL_FACE,
        DEPTH_TEST,
        POLYGON_OFFSET_FILL,
        SAMPLE_ALPHA_TO_COVERAGE,
        SCISSOR_TEST,
    ];
}

impl From<WebGLState> for u32 {
    fn from(value: WebGLState) -> Self {
        match value {
            WebGLState::Blend => BLEND,
            WebGLState::CullFace => CULL_FACE,
            WebGLState::DepthTest => DEPTH_TEST,
            WebGLState::PolygonOffsetFill => POLYGON_OFFSET_FILL,
            WebGLState::SampleAlphaToCoverage => SAMPLE_ALPHA_TO_COVERAGE,
            WebGLState::ScissorTest => SCISSOR_TEST,
        }
    }
}

impl TryFrom<u32> for WebGLState {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            BLEND => Ok(WebGLState::Blend),
            CULL_FACE => Ok(WebGLState::CullFace),
            DEPTH_TEST => Ok(WebGLState::DepthTest),
            POLYGON_OFFSET_FILL => Ok(WebGLState::PolygonOffsetFill),
            SAMPLE_ALPHA_TO_COVERAGE => Ok(WebGLState::SampleAlphaToCoverage),
            SCISSOR_TEST => Ok(WebGLState::ScissorTest),
            _ => Err(()),
        }
    }
}

impl Serialize for WebGLState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
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
                write!(f, "any of: {:?}", WebGLState::VALID_WEB_GL_STATES)
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

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate, Default)]
pub struct TechniqueParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<USize64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<StringIndex<Node>>,
    #[serde(rename = "type")]
    pub type_: Checked<ParameterType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Checked<ParameterValue>>,
}

pub const FUNC_ADD: u32 = 32774;
pub const FUNC_SUBTRACT: u32 = 32778;
pub const FUNC_REVERSE_SUBTRACT: u32 = 32779;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum BlendEquationSeparate {
    FuncAdd,
    FuncSubtract,
    FuncReverseSubtract,
}

impl BlendEquationSeparate {
    pub const VALID_BLEND_EQUATION_SEPARATES: &[u32] =
        &[FUNC_ADD, FUNC_SUBTRACT, FUNC_REVERSE_SUBTRACT];
}

impl From<BlendEquationSeparate> for u32 {
    fn from(value: BlendEquationSeparate) -> Self {
        match value {
            BlendEquationSeparate::FuncAdd => FUNC_ADD,
            BlendEquationSeparate::FuncSubtract => FUNC_SUBTRACT,
            BlendEquationSeparate::FuncReverseSubtract => FUNC_REVERSE_SUBTRACT,
        }
    }
}

impl TryFrom<u32> for BlendEquationSeparate {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            FUNC_ADD => Ok(BlendEquationSeparate::FuncAdd),
            FUNC_SUBTRACT => Ok(BlendEquationSeparate::FuncSubtract),
            FUNC_REVERSE_SUBTRACT => Ok(BlendEquationSeparate::FuncReverseSubtract),
            _ => Err(()),
        }
    }
}

impl Serialize for BlendEquationSeparate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<BlendEquationSeparate> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<BlendEquationSeparate>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "any of: {:?}",
                    BlendEquationSeparate::VALID_BLEND_EQUATION_SEPARATES
                )
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

pub const ZERO: u32 = 0;
pub const ONE: u32 = 1;
pub const SRC_COLOR: u32 = 768;
pub const ONE_MINUS_SRC_COLOR: u32 = 769;
pub const DST_COLOR: u32 = 774;
pub const ONE_MINUS_DST_COLOR: u32 = 775;
pub const SRC_ALPHA: u32 = 770;
pub const ONE_MINUS_SRC_ALPHA: u32 = 771;
pub const DST_ALPHA: u32 = 772;
pub const ONE_MINUS_DST_ALPHA: u32 = 773;
pub const CONSTANT_COLOR: u32 = 32769;
pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770;
pub const CONSTANT_ALPHA: u32 = 32771;
pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772;
pub const SRC_ALPHA_SATURATE: u32 = 776;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum BlendFuncSeparate {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
}

impl BlendFuncSeparate {
    pub const VALID_BLEND_FUNC_SEPARATES: &[u32] = &[
        ZERO,
        ONE,
        SRC_COLOR,
        ONE_MINUS_SRC_COLOR,
        DST_COLOR,
        ONE_MINUS_DST_COLOR,
        SRC_ALPHA,
        ONE_MINUS_SRC_ALPHA,
        DST_ALPHA,
        ONE_MINUS_DST_ALPHA,
        CONSTANT_COLOR,
        ONE_MINUS_CONSTANT_COLOR,
        CONSTANT_ALPHA,
        ONE_MINUS_CONSTANT_ALPHA,
        SRC_ALPHA_SATURATE,
    ];
}

impl From<BlendFuncSeparate> for u32 {
    fn from(value: BlendFuncSeparate) -> Self {
        match value {
            BlendFuncSeparate::Zero => ZERO,
            BlendFuncSeparate::One => ONE,
            BlendFuncSeparate::SrcColor => SRC_COLOR,
            BlendFuncSeparate::OneMinusSrcColor => ONE_MINUS_SRC_COLOR,
            BlendFuncSeparate::DstColor => DST_COLOR,
            BlendFuncSeparate::OneMinusDstColor => ONE_MINUS_DST_COLOR,
            BlendFuncSeparate::SrcAlpha => SRC_ALPHA,
            BlendFuncSeparate::OneMinusSrcAlpha => ONE_MINUS_SRC_ALPHA,
            BlendFuncSeparate::DstAlpha => DST_ALPHA,
            BlendFuncSeparate::OneMinusDstAlpha => ONE_MINUS_DST_ALPHA,
            BlendFuncSeparate::ConstantColor => CONSTANT_COLOR,
            BlendFuncSeparate::OneMinusConstantColor => ONE_MINUS_CONSTANT_COLOR,
            BlendFuncSeparate::ConstantAlpha => CONSTANT_ALPHA,
            BlendFuncSeparate::OneMinusConstantAlpha => ONE_MINUS_CONSTANT_ALPHA,
            BlendFuncSeparate::SrcAlphaSaturate => SRC_ALPHA_SATURATE,
        }
    }
}

impl TryFrom<u32> for BlendFuncSeparate {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            ZERO => Ok(BlendFuncSeparate::Zero),
            ONE => Ok(BlendFuncSeparate::One),
            SRC_COLOR => Ok(BlendFuncSeparate::SrcColor),
            ONE_MINUS_SRC_COLOR => Ok(BlendFuncSeparate::OneMinusSrcColor),
            DST_COLOR => Ok(BlendFuncSeparate::DstColor),
            ONE_MINUS_DST_COLOR => Ok(BlendFuncSeparate::OneMinusDstColor),
            SRC_ALPHA => Ok(BlendFuncSeparate::SrcAlpha),
            ONE_MINUS_SRC_ALPHA => Ok(BlendFuncSeparate::OneMinusSrcAlpha),
            DST_ALPHA => Ok(BlendFuncSeparate::DstAlpha),
            ONE_MINUS_DST_ALPHA => Ok(BlendFuncSeparate::OneMinusDstAlpha),
            CONSTANT_COLOR => Ok(BlendFuncSeparate::ConstantColor),
            ONE_MINUS_CONSTANT_COLOR => Ok(BlendFuncSeparate::OneMinusConstantColor),
            CONSTANT_ALPHA => Ok(BlendFuncSeparate::ConstantAlpha),
            ONE_MINUS_CONSTANT_ALPHA => Ok(BlendFuncSeparate::OneMinusConstantAlpha),
            SRC_ALPHA_SATURATE => Ok(BlendFuncSeparate::SrcAlphaSaturate),
            _ => Err(()),
        }
    }
}

impl Serialize for BlendFuncSeparate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<BlendFuncSeparate> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<BlendFuncSeparate>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "any of: {:?}",
                    BlendFuncSeparate::VALID_BLEND_FUNC_SEPARATES
                )
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

pub const FRONT: u32 = 1028;
pub const BACK: u32 = 1029;
pub const FRONT_AND_BACK: u32 = 1032;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum CullFace {
    Front,
    Back,
    FrontAndBack,
}

impl CullFace {
    pub const VALID_CULL_FACES: &[u32] = &[FRONT, BACK, FRONT_AND_BACK];
}

impl From<CullFace> for u32 {
    fn from(value: CullFace) -> Self {
        match value {
            CullFace::Front => FRONT,
            CullFace::Back => BACK,
            CullFace::FrontAndBack => FRONT_AND_BACK,
        }
    }
}

impl TryFrom<u32> for CullFace {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            FRONT => Ok(CullFace::Front),
            BACK => Ok(CullFace::Back),
            FRONT_AND_BACK => Ok(CullFace::FrontAndBack),
            _ => Err(()),
        }
    }
}

impl Serialize for CullFace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<CullFace> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<CullFace>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", CullFace::VALID_CULL_FACES)
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

pub const NEVER: u32 = 512;
pub const LESS: u32 = 513;
pub const LEQUAL: u32 = 515;
pub const EQUAL: u32 = 514;
pub const GREATER: u32 = 516;
pub const NOTEQUAL: u32 = 517;
pub const GEQUAL: u32 = 518;
pub const ALWAYS: u32 = 519;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum DepthFunc {
    Never,
    Less,
    LEqual,
    Equal,
    Greater,
    NotEqual,
    GEqual,
    Always,
}

impl DepthFunc {
    pub const VALID_DEPTH_FUNCS: &[u32] = &[
        NEVER, LESS, LEQUAL, EQUAL, GREATER, NOTEQUAL, GEQUAL, ALWAYS,
    ];
}

impl From<DepthFunc> for u32 {
    fn from(value: DepthFunc) -> Self {
        match value {
            DepthFunc::Never => NEVER,
            DepthFunc::Less => LESS,
            DepthFunc::LEqual => LEQUAL,
            DepthFunc::Equal => EQUAL,
            DepthFunc::Greater => GREATER,
            DepthFunc::NotEqual => NOTEQUAL,
            DepthFunc::GEqual => GEQUAL,
            DepthFunc::Always => ALWAYS,
        }
    }
}

impl TryFrom<u32> for DepthFunc {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            NEVER => Ok(DepthFunc::Never),
            LESS => Ok(DepthFunc::Less),
            LEQUAL => Ok(DepthFunc::LEqual),
            EQUAL => Ok(DepthFunc::Equal),
            GREATER => Ok(DepthFunc::Greater),
            NOTEQUAL => Ok(DepthFunc::NotEqual),
            GEQUAL => Ok(DepthFunc::GEqual),
            ALWAYS => Ok(DepthFunc::Always),
            _ => Err(()),
        }
    }
}

impl Serialize for DepthFunc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<DepthFunc> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<DepthFunc>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", DepthFunc::VALID_DEPTH_FUNCS)
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

pub const CW: u32 = 2304;
pub const CCW: u32 = 2305;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum FrontFace {
    CW,
    CCW,
}

impl FrontFace {
    pub const VALID_FRONT_FACES: &[u32] = &[CW, CCW];
}

impl From<FrontFace> for u32 {
    fn from(value: FrontFace) -> Self {
        match value {
            FrontFace::CW => CW,
            FrontFace::CCW => CCW,
        }
    }
}

impl TryFrom<u32> for FrontFace {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            CW => Ok(FrontFace::CW),
            CCW => Ok(FrontFace::CCW),
            _ => Err(()),
        }
    }
}

impl Serialize for FrontFace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<FrontFace> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<FrontFace>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", FrontFace::VALID_FRONT_FACES)
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
pub struct TechniqueStateFunction {
    #[serde(
        rename = "blendColor",
        skip_serializing_if = "blend_color_is_zero",
        default = "default_blend_color"
    )]
    pub blend_color: [f32; 4],
    #[serde(
        rename = "blendEquationSeparate",
        skip_serializing_if = "blend_equation_seperate_is_default",
        default = "default_blend_equation_seperate"
    )]
    pub blend_equation_seperate: [Checked<BlendEquationSeparate>; 2],
    #[serde(
        rename = "blendFuncSeparate",
        skip_serializing_if = "blend_func_seperate_is_default",
        default = "default_blend_func_seperate"
    )]
    pub blend_func_seperate: [Checked<BlendFuncSeparate>; 4],
    #[serde(
        rename = "colorMask",
        skip_serializing_if = "color_mask_is_default",
        default = "default_color_mask"
    )]
    pub color_mask: [bool; 4],
    #[serde(
        rename = "cullFace",
        skip_serializing_if = "cull_face_is_default",
        default = "default_cull_face"
    )]
    pub cull_face: [Checked<CullFace>; 1],
    #[serde(
        rename = "depthFunc",
        skip_serializing_if = "depth_func_is_default",
        default = "default_depth_func"
    )]
    pub depth_func: [Checked<DepthFunc>; 1],
    #[serde(
        rename = "depthMask",
        skip_serializing_if = "depth_mask_is_default",
        default = "default_depth_mask"
    )]
    pub depth_mask: [bool; 1],
    #[serde(
        rename = "depthRange",
        skip_serializing_if = "depth_range_is_default",
        default = "default_depth_range"
    )]
    pub depth_range: [f32; 2],
    #[serde(
        rename = "frontFace",
        skip_serializing_if = "front_face_is_default",
        default = "default_front_face"
    )]
    pub front_face: [Checked<FrontFace>; 1],
    #[serde(
        rename = "lineWidth",
        skip_serializing_if = "line_width_is_default",
        default = "default_line_width"
    )]
    pub line_width: [f32; 1],
    #[serde(
        rename = "polygonOffset",
        skip_serializing_if = "polygon_offset_is_default",
        default = "default_polygon_offset"
    )]
    pub polygon_offset: [f32; 2],
    #[serde(
        skip_serializing_if = "scissor_is_default",
        default = "default_scissor"
    )]
    pub scissor: [f32; 4],
}

fn blend_color_is_zero(value: &[f32; 4]) -> bool {
    value[0] == 0.0 && value[1] == 0.0 && value[2] == 0.0 && value[3] == 0.0
}

fn default_blend_color() -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
}

fn blend_equation_seperate_is_default(value: &[Checked<BlendEquationSeparate>; 2]) -> bool {
    value[0] == Checked::Valid(BlendEquationSeparate::FuncAdd)
        && value[1] == Checked::Valid(BlendEquationSeparate::FuncAdd)
}

fn default_blend_equation_seperate() -> [Checked<BlendEquationSeparate>; 2] {
    [
        Checked::Valid(BlendEquationSeparate::FuncAdd),
        Checked::Valid(BlendEquationSeparate::FuncAdd),
    ]
}
fn blend_func_seperate_is_default(value: &[Checked<BlendFuncSeparate>; 4]) -> bool {
    value[0] == Checked::Valid(BlendFuncSeparate::One)
        && value[1] == Checked::Valid(BlendFuncSeparate::Zero)
        && value[2] == Checked::Valid(BlendFuncSeparate::One)
        && value[3] == Checked::Valid(BlendFuncSeparate::Zero)
}

fn default_blend_func_seperate() -> [Checked<BlendFuncSeparate>; 4] {
    [
        Checked::Valid(BlendFuncSeparate::One),
        Checked::Valid(BlendFuncSeparate::Zero),
        Checked::Valid(BlendFuncSeparate::One),
        Checked::Valid(BlendFuncSeparate::Zero),
    ]
}

fn color_mask_is_default(value: &[bool; 4]) -> bool {
    value[0] && value[1] && value[2] && value[3]
}

fn default_color_mask() -> [bool; 4] {
    [true, true, true, true]
}

fn cull_face_is_default(value: &[Checked<CullFace>; 1]) -> bool {
    value[0] == Checked::Valid(CullFace::Back)
}

fn default_cull_face() -> [Checked<CullFace>; 1] {
    [Checked::Valid(CullFace::Back)]
}

fn depth_func_is_default(value: &[Checked<DepthFunc>; 1]) -> bool {
    value[0] == Checked::Valid(DepthFunc::Less)
}

fn default_depth_func() -> [Checked<DepthFunc>; 1] {
    [Checked::Valid(DepthFunc::Less)]
}

fn depth_mask_is_default(value: &[bool; 1]) -> bool {
    value[0] == true
}

fn default_depth_mask() -> [bool; 1] {
    [true]
}

fn depth_range_is_default(value: &[f32; 2]) -> bool {
    value[0] == 0.0 && value[1] == 1.0
}

fn default_depth_range() -> [f32; 2] {
    [0.0, 1.0]
}

fn front_face_is_default(value: &[Checked<FrontFace>; 1]) -> bool {
    value[0] == Checked::Valid(FrontFace::CCW)
}

fn default_front_face() -> [Checked<FrontFace>; 1] {
    [Checked::Valid(FrontFace::CCW)]
}

fn line_width_is_default(value: &[f32; 1]) -> bool {
    value[0] == 1.0
}

fn default_line_width() -> [f32; 1] {
    [1.0]
}

fn polygon_offset_is_default(value: &[f32; 2]) -> bool {
    value[0] == 0.0 && value[1] == 0.0
}

fn default_polygon_offset() -> [f32; 2] {
    [0.0, 0.0]
}

fn scissor_is_default(value: &[f32; 4]) -> bool {
    value[0] == 0.0 && value[1] == 0.0 && value[2] == 0.0 && value[3] == 0.0
}

fn default_scissor() -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct TechniqueState {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enable: Vec<Checked<WebGLState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<TechniqueStateFunction>,
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
#[gltf(validate_hook = "technique_validate_technique")]
pub struct Technique {
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<String, TechniqueParameter>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub attributes: IndexMap<String, String>,
    pub program: StringIndex<Program>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub uniforms: IndexMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<TechniqueState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Technique {
    pub(crate) fn default_technique(program: String) -> Self {
        let mut attributes = IndexMap::new();
        attributes.insert("a_position".to_string(), "position".to_string());

        let mut parameters = IndexMap::new();
        parameters.insert(
            "modelViewMatrix".to_string(),
            TechniqueParameter {
                semantic: Some("MODELVIEW".to_string()),
                type_: Checked::Valid(ParameterType::FloatMat4),
                ..TechniqueParameter::default()
            },
        );
        parameters.insert(
            "projectionMatrix".to_string(),
            TechniqueParameter {
                semantic: Some("PROJECTION".to_string()),
                type_: Checked::Valid(ParameterType::FloatMat4),
                ..TechniqueParameter::default()
            },
        );
        parameters.insert(
            "emission".to_string(),
            TechniqueParameter {
                value: Some(Checked::Valid(ParameterValue::NumberArray(vec![
                    0.5, 0.5, 0.5, 1.0,
                ]))),
                type_: Checked::Valid(ParameterType::FloatVec4),
                ..TechniqueParameter::default()
            },
        );
        parameters.insert(
            "position".to_string(),
            TechniqueParameter {
                semantic: Some("POSITION".to_string()),
                type_: Checked::Valid(ParameterType::FloatVec3),
                ..TechniqueParameter::default()
            },
        );

        let mut uniforms = IndexMap::new();
        uniforms.insert(
            "u_modelViewMatrix".to_string(),
            "modelViewMatrix".to_string(),
        );
        uniforms.insert(
            "u_projectionMatrix".to_string(),
            "projectionMatrix".to_string(),
        );
        uniforms.insert("u_emission".to_string(), "emission".to_string());

        Technique {
            attributes,
            parameters,
            program: StringIndex::new(program),
            uniforms,
            states: Some(TechniqueState {
                enable: vec![
                    Checked::Valid(WebGLState::CullFace),
                    Checked::Valid(WebGLState::DepthTest),
                ],
                functions: None,
            }),
            name: None,
        }
    }
}
fn technique_validate_technique<P, R>(technique: &Technique, _root: &Root, path: P, report: &mut R)
where
    P: Fn() -> Path,
    R: FnMut(&dyn Fn() -> Path, Error),
{
    for (_, parameter_key) in &technique.attributes {
        if !technique.parameters.contains_key(parameter_key) {
            report(&path, Error::IndexNotFound);
        }
    }
    for (_, parameter_key) in &technique.uniforms {
        if !technique.parameters.contains_key(parameter_key) {
            report(&path, Error::IndexNotFound);
        }
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technique: Option<StringIndex<Technique>>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub values: IndexMap<String, Checked<ParameterValue>>,
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
