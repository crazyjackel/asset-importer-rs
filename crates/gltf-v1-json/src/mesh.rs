use std::fmt::{self, Display};

use gltf_v1_derive::Validate;
use indexmap::IndexMap;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use super::{accessor::Accessor, common::StringIndex, material::Material, validation::Checked};

pub const POSITION: &str = "POSITION";
pub const NORMAL: &str = "NORMAL";
pub const COLOR: &str = "COLOR_";
pub const TEXCOORD: &str = "TEXCOORD_";
pub const JOINT: &str = "JOINT_";
pub const WEIGHT: &str = "WEIGHT_";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Semantic {
    Positions,
    Normals,
    Colors(u32),
    TexCoords(u32),
    Joints(u32),
    Weights(u32),
}

impl Semantic {
    pub const VALID_SEMANTICS: &[&str] = &[POSITION, NORMAL, COLOR, TEXCOORD, JOINT, WEIGHT];
}

impl Display for Semantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String = (*self).into();
        f.write_str(&str)
    }
}

impl From<Semantic> for String {
    fn from(value: Semantic) -> Self {
        match value {
            Semantic::Positions => POSITION.into(),
            Semantic::Normals => NORMAL.into(),
            Semantic::Colors(c) => format!("{}{}", COLOR, c),
            Semantic::TexCoords(c) => format!("{}{}", TEXCOORD, c),
            Semantic::Joints(c) => format!("{}{}", JOINT, c),
            Semantic::Weights(c) => format!("{}{}", WEIGHT, c),
        }
    }
}

impl TryFrom<&str> for Semantic {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            NORMAL => Ok(Semantic::Normals),
            POSITION => Ok(Semantic::Positions),
            _ if s.starts_with(COLOR) => match s[COLOR.len()..].parse() {
                Ok(set) => Ok(Semantic::Colors(set)),
                Err(_) => Err(()),
            },
            _ if s.starts_with(TEXCOORD) => match s[TEXCOORD.len()..].parse() {
                Ok(set) => Ok(Semantic::TexCoords(set)),
                Err(_) => Err(()),
            },
            _ if s.starts_with(JOINT) => match s[JOINT.len()..].parse() {
                Ok(set) => Ok(Semantic::Joints(set)),
                Err(_) => Err(()),
            },
            _ if s.starts_with(WEIGHT) => match s[WEIGHT.len()..].parse() {
                Ok(set) => Ok(Semantic::Weights(set)),
                Err(_) => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl ser::Serialize for Semantic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> de::Deserialize<'de> for Checked<Semantic> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Semantic>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", Semantic::VALID_SEMANTICS)
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
        deserializer.deserialize_str(Visitor)
    }
}
pub const POINTS: u32 = 0;
pub const LINES: u32 = 1;
pub const LINE_LOOP: u32 = 2;
pub const LINE_STRIP: u32 = 3;
pub const TRIANGLES: u32 = 4;
pub const TRIANGLE_STRIP: u32 = 5;
pub const TRIANGLE_FAN: u32 = 6;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveMode {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl PrimitiveMode {
    pub const VALID_PRIMITIVE_MODES: &[u32] = &[
        POINTS,
        LINES,
        LINE_LOOP,
        LINE_STRIP,
        TRIANGLES,
        TRIANGLE_STRIP,
        TRIANGLE_FAN,
    ];
}

impl From<PrimitiveMode> for u32 {
    fn from(value: PrimitiveMode) -> Self {
        match value {
            PrimitiveMode::Points => POINTS,
            PrimitiveMode::Lines => LINES,
            PrimitiveMode::LineLoop => LINE_LOOP,
            PrimitiveMode::LineStrip => LINE_STRIP,
            PrimitiveMode::Triangles => TRIANGLES,
            PrimitiveMode::TriangleStrip => TRIANGLE_STRIP,
            PrimitiveMode::TriangleFan => TRIANGLE_FAN,
        }
    }
}

impl TryFrom<u32> for PrimitiveMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            POINTS => Ok(PrimitiveMode::Points),
            LINES => Ok(PrimitiveMode::Lines),
            LINE_LOOP => Ok(PrimitiveMode::LineLoop),
            LINE_STRIP => Ok(PrimitiveMode::LineStrip),
            TRIANGLES => Ok(PrimitiveMode::Triangles),
            TRIANGLE_STRIP => Ok(PrimitiveMode::TriangleStrip),
            TRIANGLE_FAN => Ok(PrimitiveMode::TriangleFan),
            _ => Err(()),
        }
    }
}

impl ser::Serialize for PrimitiveMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u32((*self).into())
    }
}

impl<'de> de::Deserialize<'de> for Checked<PrimitiveMode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<PrimitiveMode>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", PrimitiveMode::VALID_PRIMITIVE_MODES)
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

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Primitive {
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub attributes: IndexMap<Checked<Semantic>, StringIndex<Accessor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<StringIndex<Accessor>>,
    pub material: StringIndex<Material>,
    #[serde(default = "default_primitive_mode")]
    pub mode: Checked<PrimitiveMode>,
}

impl Primitive {
    pub fn new(material: StringIndex<Material>) -> Self {
        Self {
            attributes: IndexMap::new(),
            indices: None,
            material,
            mode: Checked::Valid(PrimitiveMode::Triangles),
        }
    }
}

fn default_primitive_mode() -> Checked<PrimitiveMode> {
    Checked::Valid(PrimitiveMode::Triangles)
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, Default)]
pub struct Mesh {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub primitives: Vec<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[test]
fn test_mesh_deserialize() {
    let data = r#"{
            "name": "user-defined name of mesh",
            "primitives": [
                {
                    "attributes": {
                        "NORMAL": "accessor_id0",
                        "POSITION": "accessor_id1",
                        "TEXCOORD_0": "accessor_id2"
                    },
                    "indices": "accessor_id3",
                    "material": "material_id",
                    "mode": 4,
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
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }     
        }"#;
    let mesh: Result<Mesh, _> = serde_json::from_str(data);
    let mesh_unwrap = mesh.unwrap();
    println!("{}", serde_json::to_string(&mesh_unwrap).unwrap());
    assert_eq!(
        Some("user-defined name of mesh".to_string()),
        mesh_unwrap.name
    );
}
