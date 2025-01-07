use serde_derive::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use serde::de;
use std::fmt;

use super::validation::Checked;

/// Corresponds to `GL_ARRAY_BUFFER`.
pub const ARRAY_BUFFER: u32 = 34_962;

/// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
pub const ELEMENT_ARRAY_BUFFER: u32 = 34_963;

pub const VALID_TARGETS: &[u32] = &[ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BufferType {
    #[serde(rename = "arraybuffer")]
    ArrayBuffer,
    #[serde(rename = "text")]
    Text
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferViewType{
    ArrayBuffer,
    ElementArrayBuffer
}


impl Serialize for BufferViewType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match *self {
                BufferViewType::ArrayBuffer => serializer.serialize_u32(ARRAY_BUFFER),
                BufferViewType::ElementArrayBuffer => serializer.serialize_u32(ELEMENT_ARRAY_BUFFER),
            }
    }
}

impl<'de> Deserialize<'de> for Checked<BufferViewType>{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Checked<BufferViewType>;
    
                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "any of: {:?}", VALID_TARGETS)
                }
    
                fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    use self::BufferViewType::*;
                    Ok(match value as u32 {
                        ARRAY_BUFFER => Checked::Valid(ArrayBuffer),
                        ELEMENT_ARRAY_BUFFER => Checked::Valid(ElementArrayBuffer),
                        _ => Checked::Invalid,
                    })
                }
            }
            deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Buffer {
    pub uri: String,
    #[serde(rename = "byteLength", skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<usize>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub buffer_type: Option<BufferType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BufferView {
    buffer: String,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteLength", skip_serializing_if = "Option::is_none")]
    byte_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Checked<BufferViewType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
}