
use std::{collections::BTreeMap, fmt};

use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use super::{accessor::Accessor, material::Material, root::StringIndex, validation::Checked};

pub const POINTS: u32 = 0;
pub const LINES: u32 = 1;
pub const LINE_LOOP: u32 = 2;
pub const LINE_STRIP: u32 = 3;
pub const TRIANGLES: u32 = 4;
pub const TRIANGLE_STRIP: u32 = 5;
pub const TRIANGLE_FAN: u32 = 6;

pub const VALID_PRIMITIVE_MODES: &[u32] = &[
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrimitiveMode {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl PrimitiveMode {
    pub const fn to_repr(self) -> u32 {
        match self {
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
impl ser::Serialize for PrimitiveMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u32(self.to_repr())
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
                write!(f, "any of: {:?}", VALID_PRIMITIVE_MODES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::PrimitiveMode::*;
                Ok(match value as u32 {
                    POINTS => Checked::Valid(Points),
                    LINES => Checked::Valid(Lines),
                    LINE_LOOP => Checked::Valid(LineLoop),
                    LINE_STRIP => Checked::Valid(LineStrip),
                    TRIANGLES => Checked::Valid(Triangles),
                    TRIANGLE_STRIP => Checked::Valid(TriangleStrip),
                    TRIANGLE_FAN => Checked::Valid(TriangleFan),
                    _ => Checked::Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Primitive{
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<BTreeMap<String, StringIndex<Accessor>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indices: Option<StringIndex<Accessor>>,
    material: StringIndex<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<Checked<PrimitiveMode>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh{
    #[serde(skip_serializing_if = "Option::is_none")]
    primitives: Option<Vec<Primitive>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>
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
