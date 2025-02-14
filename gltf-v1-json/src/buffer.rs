use gltf_v1_derive::Validate;
use serde::de;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::validation::USize64;

use super::validation::Checked;

/// Corresponds to `GL_ARRAY_BUFFER`.
pub const ARRAY_BUFFER: u32 = 34_962;

/// Corresponds to `GL_ELEMENT_ARRAY_BUFFER`.
pub const ELEMENT_ARRAY_BUFFER: u32 = 34_963;

pub const ARRAY_BUFFER_TYPE: &str = "arraybuffer";

pub const TEXT_TYPE: &str = "text";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum BufferType {
    #[default]
    ArrayBuffer,
    Text,
}

impl BufferType {
    pub const VALID_TYPES: &[&str] = &[ARRAY_BUFFER_TYPE, TEXT_TYPE];
}

impl TryFrom<&str> for BufferType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ARRAY_BUFFER_TYPE => Ok(BufferType::ArrayBuffer),
            TEXT_TYPE => Ok(BufferType::Text),
            _ => Err(()),
        }
    }
}

impl From<BufferType> for &str {
    fn from(value: BufferType) -> Self {
        match value {
            BufferType::ArrayBuffer => ARRAY_BUFFER_TYPE,
            BufferType::Text => TEXT_TYPE,
        }
    }
}

impl Serialize for BufferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<BufferType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<BufferType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", BufferType::VALID_TYPES)
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
        deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum BufferViewType {
    #[default]
    ArrayBuffer,
    ElementArrayBuffer,
}

impl BufferViewType {
    pub const VALID_TARGETS: &[u32] = &[ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER];
}

impl TryFrom<u32> for BufferViewType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            ARRAY_BUFFER => Ok(BufferViewType::ArrayBuffer),
            ELEMENT_ARRAY_BUFFER => Ok(BufferViewType::ElementArrayBuffer),
            _ => Err(()),
        }
    }
}

impl From<BufferViewType> for u32 {
    fn from(value: BufferViewType) -> Self {
        match value {
            BufferViewType::ArrayBuffer => ARRAY_BUFFER,
            BufferViewType::ElementArrayBuffer => ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl Serialize for BufferViewType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> Deserialize<'de> for Checked<BufferViewType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checked<BufferViewType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", BufferViewType::VALID_TARGETS)
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
        deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct Buffer {
    pub uri: String,
    #[serde(rename = "byteLength")]
    pub byte_length: USize64,
    #[serde(rename = "type")]
    pub type_: Option<Checked<BufferType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize, Validate)]
pub struct BufferView {
    pub buffer: String,
    #[serde(rename = "byteOffset")]
    pub byte_offset: USize64,
    #[serde(rename = "byteLength", skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<USize64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Checked<BufferViewType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[test]
fn test_buffer_deserialize() {
    let data = r#"{
            "uri": "vertices.bin",
            "byteLength": 1024,
            "name": "user-defined buffer name",
            "type": "arraybuffer",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }
        }"#;
    let buffer: Result<Buffer, _> = serde_json::from_str(data);
    let buffer_unwrap = buffer.unwrap();
    println!("{}", serde_json::to_string(&buffer_unwrap).unwrap());
    assert_eq!("vertices.bin".to_string(), buffer_unwrap.uri);
}

#[test]
fn test_buffer_view_deserialize() {
    let data = r#"{
            "buffer" : "buffer_id",
            "byteLength": 76768,
            "byteOffset": 0,
            "name": "user-defined name of bufferView with vertices",
            "target": 34962,
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }
        }"#;
    let buffer_view: Result<BufferView, _> = serde_json::from_str(data);
    let buffer_view_unwrap = buffer_view.unwrap();
    println!("{}", serde_json::to_string(&buffer_view_unwrap).unwrap());
    assert_eq!(
        Some("user-defined name of bufferView with vertices".to_string()),
        buffer_view_unwrap.name
    );
}
