use std::fmt;

use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use super::validation::Checked;

/// All valid camera types.
pub const VALID_CAMERA_TYPES: &[&str] = &["perspective", "orthographic"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perspective {
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<f32>,
    yfov: f32,
    zfar: f32,
    znear: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ortographic {
    xmag: f32,
    ymag: f32,
    zfar: f32,
    znear: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CameraType {
    Perspective,
    Orthographic,
}

impl<'de> de::Deserialize<'de> for Checked<CameraType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<CameraType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_CAMERA_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::CameraType::*;
                use super::validation::Checked::*;
                Ok(match value {
                    "perspective" => Valid(Perspective),
                    "orthographic" => Valid(Orthographic),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ser::Serialize for CameraType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            CameraType::Perspective => serializer.serialize_str("perspective"),
            CameraType::Orthographic => serializer.serialize_str("orthographic"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Camera {
    #[serde(skip_serializing_if = "Option::is_none")]
    orthographic: Option<Ortographic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perspective: Option<Perspective>,
    #[serde(rename = "type")]
    camera_type: Checked<CameraType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[test]
fn test_camera_deserialize() {
    let data = r#"{
            "name" : "user-defined name of perspective camera",
            "perspective" : {
                "aspectRatio" : 1.77,
                "yfov" : 0.7854,
                "zfar" : 1000,
                "znear" : 1
            },
            "type" : "perspective",
            "extensions" : {
               "extension_name" : {
                  "extension specific" : "value"
               }
            },
            "extras" : {
                "Application specific" : "The extra object can contain any properties."
            }
        }"#;
    let camera: Result<Camera, _> = serde_json::from_str(data);
    let camera_unwrap = camera.unwrap();
    println!("{}", serde_json::to_string(&camera_unwrap).unwrap());
    assert_eq!(
        Some("user-defined name of perspective camera".to_string()),
        camera_unwrap.name
    );
}
