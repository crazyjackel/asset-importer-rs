use std::fmt;

use gltf_v1_derive::Validate;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};

use super::validation::Checked;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Perspective {
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f32>,
    pub yfov: f32,
    pub zfar: f32,
    pub znear: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Orthographic {
    pub xmag: f32,
    pub ymag: f32,
    pub zfar: f32,
    pub znear: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CameraType {
    Perspective,
    Orthographic,
}

impl CameraType {
    pub const VALID_CAMERA_TYPES: &[&str] = &["perspective", "orthographic"];
}

impl TryFrom<&str> for CameraType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "perspective" => Ok(CameraType::Perspective),
            "orthographic" => Ok(CameraType::Orthographic),
            _ => Err(()),
        }
    }
}

impl From<CameraType> for &str {
    fn from(value: CameraType) -> Self {
        match value {
            CameraType::Perspective => "perspective",
            CameraType::Orthographic => "orthographic",
        }
    }
}

impl ser::Serialize for CameraType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str((*self).into())
    }
}

impl<'de> de::Deserialize<'de> for Checked<CameraType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl de::Visitor<'_> for Visitor {
            type Value = Checked<CameraType>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", CameraType::VALID_CAMERA_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value
                    .try_into()
                    .map(Checked::Valid)
                    .unwrap_or(Checked::Invalid))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Camera {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orthographic: Option<Orthographic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perspective: Option<Perspective>,
    #[serde(rename = "type")]
    pub type_: Checked<CameraType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
