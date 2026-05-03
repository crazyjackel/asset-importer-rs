//! FBX `NodeAttribute` / `Camera` — Assimp [`Camera`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

const PROP_POSITION: &str = "Position";
const PROP_UP_VECTOR: &str = "UpVector";
const PROP_INTEREST_POSITION: &str = "InterestPosition";
const PROP_ASPECT_WIDTH: &str = "AspectWidth";
const PROP_ASPECT_HEIGHT: &str = "AspectHeight";
const PROP_FILM_WIDTH: &str = "FilmWidth";
const PROP_FILM_HEIGHT: &str = "FilmHeight";
const PROP_NEAR_PLANE: &str = "NearPlane";
const PROP_FAR_PLANE: &str = "FarPlane";
const PROP_FILM_ASPECT_RATIO: &str = "FilmAspectRatio";
const PROP_APERTURE_MODE: &str = "ApertureMode";
const PROP_FIELD_OF_VIEW: &str = "FieldOfView";
const PROP_FOCAL_LENGTH: &str = "FocalLength";

const DEFAULT_FOV_UNKNOWN: f32 = -1.0;

#[derive(Debug, PartialEq)]
pub struct Camera(pub OwnedObject);

impl Camera {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }

    /// Temporary bridge to Assimp-style camera properties until typed accessors are added.
    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.0.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.0.properties.get(name)
    }

    // Assimp parity (`FBXDocument.h` camera accessors) from NodeAttribute property table.
    pub fn position(&self) -> [f32; 3] {
        match self.property(PROP_POSITION) {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }

    pub fn up_vector(&self) -> [f32; 3] {
        match self.property(PROP_UP_VECTOR) {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 1.0, 0.0],
        }
    }

    pub fn interest_position(&self) -> [f32; 3] {
        match self.property(PROP_INTEREST_POSITION) {
            Some(Property::Vec3(v)) => *v,
            _ => [0.0, 0.0, 0.0],
        }
    }

    pub fn aspect_width(&self) -> f32 {
        match self.property(PROP_ASPECT_WIDTH) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn aspect_height(&self) -> f32 {
        match self.property(PROP_ASPECT_HEIGHT) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn film_width(&self) -> f32 {
        match self.property(PROP_FILM_WIDTH) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn film_height(&self) -> f32 {
        match self.property(PROP_FILM_HEIGHT) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn near_plane(&self) -> f32 {
        match self.property(PROP_NEAR_PLANE) {
            Some(Property::Float(v)) => *v,
            _ => 0.1,
        }
    }

    pub fn far_plane(&self) -> f32 {
        match self.property(PROP_FAR_PLANE) {
            Some(Property::Float(v)) => *v,
            _ => 100.0,
        }
    }

    pub fn film_aspect_ratio(&self) -> f32 {
        match self.property(PROP_FILM_ASPECT_RATIO) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }

    pub fn aperture_mode(&self) -> i32 {
        match self.property(PROP_APERTURE_MODE) {
            Some(Property::Int(v)) => *v,
            _ => 0,
        }
    }

    pub fn field_of_view(&self) -> f32 {
        match self.property(PROP_FIELD_OF_VIEW) {
            Some(Property::Float(v)) => *v,
            _ => DEFAULT_FOV_UNKNOWN,
        }
    }

    pub fn focal_length(&self) -> f32 {
        match self.property(PROP_FOCAL_LENGTH) {
            Some(Property::Float(v)) => *v,
            _ => 1.0,
        }
    }
}

impl TryFrom<OwnedObject> for Camera {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::Camera => Ok(Camera(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "Camera".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{NODE_ATTRIBUTE_CAMERA_CLASS_NAME, NODE_ATTRIBUTE_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::Camera;

    #[test]
    fn property_accessors_return_owned_object_properties() {
        let mut properties = HashMap::new();
        properties.insert("NearPlane".to_string(), Property::Float(0.25));
        let o = OwnedObject {
            object_index: 99,
            name: "Camera".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_CAMERA_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let camera = Camera::try_from(o).unwrap();
        assert_eq!(camera.property("NearPlane"), Some(&Property::Float(0.25)));
        assert_eq!(camera.properties().len(), 1);
    }

    #[test]
    fn typed_accessors_return_defaults() {
        let o = OwnedObject {
            object_index: 99,
            name: "Camera".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_CAMERA_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let camera = Camera::try_from(o).unwrap();
        assert_eq!(camera.position(), [0.0, 0.0, 0.0]);
        assert_eq!(camera.up_vector(), [0.0, 1.0, 0.0]);
        assert_eq!(camera.interest_position(), [0.0, 0.0, 0.0]);
        assert_eq!(camera.aspect_width(), 1.0);
        assert_eq!(camera.aspect_height(), 1.0);
        assert_eq!(camera.film_width(), 1.0);
        assert_eq!(camera.film_height(), 1.0);
        assert_eq!(camera.near_plane(), 0.1);
        assert_eq!(camera.far_plane(), 100.0);
        assert_eq!(camera.film_aspect_ratio(), 1.0);
        assert_eq!(camera.aperture_mode(), 0);
        assert_eq!(camera.field_of_view(), -1.0);
        assert_eq!(camera.focal_length(), 1.0);
    }

    #[test]
    fn typed_accessors_return_property_values() {
        let mut properties = HashMap::new();
        properties.insert("Position".to_string(), Property::Vec3([1.0, 2.0, 3.0]));
        properties.insert("UpVector".to_string(), Property::Vec3([0.0, 0.0, 1.0]));
        properties.insert(
            "InterestPosition".to_string(),
            Property::Vec3([4.0, 5.0, 6.0]),
        );
        properties.insert("AspectWidth".to_string(), Property::Float(16.0));
        properties.insert("AspectHeight".to_string(), Property::Float(9.0));
        properties.insert("FilmWidth".to_string(), Property::Float(0.825));
        properties.insert("FilmHeight".to_string(), Property::Float(0.446));
        properties.insert("NearPlane".to_string(), Property::Float(0.25));
        properties.insert("FarPlane".to_string(), Property::Float(500.0));
        properties.insert("FilmAspectRatio".to_string(), Property::Float(1.777_777_8));
        properties.insert("ApertureMode".to_string(), Property::Int(1));
        properties.insert("FieldOfView".to_string(), Property::Float(60.0));
        properties.insert("FocalLength".to_string(), Property::Float(35.0));
        let o = OwnedObject {
            object_index: 99,
            name: "Camera".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_CAMERA_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let camera = Camera::try_from(o).unwrap();
        assert_eq!(camera.position(), [1.0, 2.0, 3.0]);
        assert_eq!(camera.up_vector(), [0.0, 0.0, 1.0]);
        assert_eq!(camera.interest_position(), [4.0, 5.0, 6.0]);
        assert_eq!(camera.aspect_width(), 16.0);
        assert_eq!(camera.aspect_height(), 9.0);
        assert_eq!(camera.film_width(), 0.825);
        assert_eq!(camera.film_height(), 0.446);
        assert_eq!(camera.near_plane(), 0.25);
        assert_eq!(camera.far_plane(), 500.0);
        assert_eq!(camera.film_aspect_ratio(), 1.777_777_8);
        assert_eq!(camera.aperture_mode(), 1);
        assert_eq!(camera.field_of_view(), 60.0);
        assert_eq!(camera.focal_length(), 35.0);
    }
}
