use std::fmt;

use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use super::{buffer::BufferView, root::StringIndex, validation::Checked};

/// Corresponds to `GL_BYTE`.
pub const BYTE: u32 = 5120;

/// Corresponds to `GL_UNSIGNED_BYTE`.
pub const UNSIGNED_BYTE: u32 = 5121;

/// Corresponds to `GL_SHORT`.
pub const SHORT: u32 = 5122;

/// Corresponds to `GL_UNSIGNED_SHORT`.
pub const UNSIGNED_SHORT: u32 = 5123;

/// Corresponds to `GL_UNSIGNED_INT`.
pub const UNSIGNED_INT: u32 = 5125;

/// Corresponds to `GL_FLOAT`.
pub const FLOAT: u32 = 5126;

pub const VALID_COMPONENT_TYPES: &[u32] = &[
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    UNSIGNED_INT,
    FLOAT,
];

pub const VALID_ACCESSOR_TYPES: &[&str] =
    &["SCALAR", "VEC2", "VEC3", "VEC4", "MAT2", "MAT3", "MAT4"];
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float,
}

impl ComponentType {
    pub const fn size(&self) -> u32 {
        match self {
            ComponentType::Byte | ComponentType::UnsignedByte => 1,
            ComponentType::Short | ComponentType::UnsignedShort => 2,
            ComponentType::UnsignedInt | ComponentType::Float => 4,
        }
    }
    pub const fn to_repr(self) -> u32 {
        match self {
            ComponentType::Byte => BYTE,
            ComponentType::UnsignedByte => UNSIGNED_BYTE,
            ComponentType::Short => SHORT,
            ComponentType::UnsignedShort => UNSIGNED_SHORT,
            ComponentType::UnsignedInt => UNSIGNED_INT,
            ComponentType::Float => FLOAT,
        }
    }
}

impl ser::Serialize for ComponentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
    }
}

impl<'de> de::Deserialize<'de> for Checked<ComponentType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<ComponentType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_COMPONENT_TYPES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::ComponentType::*;
                Ok(match value as u32 {
                    BYTE => Checked::Valid(Byte),
                    UNSIGNED_BYTE => Checked::Valid(UnsignedByte),
                    SHORT => Checked::Valid(Short),
                    UNSIGNED_SHORT => Checked::Valid(UnsignedShort),
                    UNSIGNED_INT => Checked::Valid(UnsignedInt),
                    FLOAT => Checked::Valid(Float),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    MAT2,
    MAT3,
    MAT4,
}

impl Type {
    pub const fn get_num_components(&self) -> u32 {
        match self {
            Type::SCALAR => 1,
            Type::VEC2 => 2,
            Type::VEC3 => 3,
            Type::VEC4 => 4,
            Type::MAT2 => 4,
            Type::MAT3 => 9,
            Type::MAT4 => 16,
        }
    }
    pub fn from_str(str: &str) -> Type {
        match str {
            "SCALAR" => Type::SCALAR,
            "VEC2" => Type::VEC2,
            "VEC3" => Type::VEC3,
            "VEC4" => Type::VEC4,
            "MAT2" => Type::MAT2,
            "MAT3" => Type::MAT3,
            "MAT4" => Type::MAT4,
            _ => Type::SCALAR,
        }
    }
}

impl ser::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(match *self {
            Type::SCALAR => "SCALAR",
            Type::VEC2 => "VEC2",
            Type::VEC3 => "VEC3",
            Type::VEC4 => "VEC4",
            Type::MAT2 => "MAT2",
            Type::MAT3 => "MAT3",
            Type::MAT4 => "MAT4",
        })
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
                write!(f, "any of: {:?}", VALID_ACCESSOR_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::Type::*;
                Ok(match value {
                    "SCALAR" => Checked::Valid(SCALAR),
                    "VEC2" => Checked::Valid(VEC2),
                    "VEC3" => Checked::Valid(VEC3),
                    "VEC4" => Checked::Valid(VEC4),
                    "MAT2" => Checked::Valid(MAT2),
                    "MAT3" => Checked::Valid(MAT3),
                    "MAT4" => Checked::Valid(MAT4),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accessor {
    #[serde(rename = "bufferView")]
    buffer_view: StringIndex<BufferView>,
    #[serde(rename = "byteOffset")]
    byte_offset: u32,
    #[serde(rename = "byteStride", skip_serializing_if = "Option::is_none")]
    byte_stride: Option<u32>,
    #[serde(rename = "componentType")]
    component_type: Checked<ComponentType>,
    count: u32,
    #[serde(rename = "type")]
    attribute_type: Checked<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[test]
fn test_accessor_deserialize() {
    let data = r#"{
            "bufferView" : "bufferViewWithVertices_id",
            "byteOffset" : 0,
            "byteStride" : 3,
            "componentType" : 5126,
            "count" : 1024,
            "type" : "SCALAR",
            "name": "user-defined accessor name",
            "max" : [-1.0, -1.0, -1.0],
            "min" : [1.0, 1.0, 1.0],
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }
    "#;
    let accessor: Result<Accessor, _> = serde_json::from_str(data);
    let accessor_unwrap = accessor.unwrap();
    println!("{}", serde_json::to_string(&accessor_unwrap).unwrap());
    assert_eq!(
        Some("user-defined accessor name".to_string()),
        accessor_unwrap.name
    );
}
