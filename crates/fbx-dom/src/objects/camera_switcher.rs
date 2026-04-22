//! FBX `NodeAttribute` / `CameraSwitcher` — Assimp [`CameraSwitcher`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXNodeAttribute.cpp).
//!
//! Unlike Assimp (optional `CameraId` / `CameraName` / `CameraIndexName`), we require all three
//! child elements with a single value token each.

use std::collections::HashMap;
use std::convert::TryFrom;

use fbxscii::ElementAttribute;

use crate::{OwnedObject, Property, objects::AttrExtractorExt};

use super::{
    fbx_object_tag, FbxObjectTag, FbxTryFromReason, FbxTypeMismatch,
};

const CAMERA_ID: &str = "CameraId";
const CAMERA_NAME: &str = "CameraName";
const CAMERA_INDEX_NAME: &str = "CameraIndexName";

#[derive(Debug, PartialEq)]
pub struct CameraSwitcher {
    object: OwnedObject,
    pub camera_id: i32,
    pub camera_name: String,
    pub camera_index_name: String,
}

impl CameraSwitcher {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }

    /// Temporary bridge to Assimp-style camera switcher properties until typed accessors are added.
    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.object.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.object.properties.get(name)
    }
}

fn parse_camera_switcher_fields(
    attrs: &HashMap<String, ElementAttribute>,
) -> Result<(i32, String, String), FbxTryFromReason> {
    let id_tok = attrs.require_token(&CAMERA_ID)?;
    let camera_id = id_tok.parse::<i32>().map_err(|e| FbxTryFromReason::InvalidAttributeFormat {
        name: CAMERA_ID.to_string(),
        detail: e.to_string(),
    })?;

    let camera_name = attrs.require_token(&CAMERA_NAME)?.to_string();
    let camera_index_name = attrs.require_token(&CAMERA_INDEX_NAME)?.to_string();

    Ok((camera_id, camera_name, camera_index_name))
}

impl TryFrom<OwnedObject> for CameraSwitcher {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::CameraSwitcher) {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "CameraSwitcher".to_string()));
        }

        match parse_camera_switcher_fields(&o.attributes) {
            Ok((camera_id, camera_name, camera_index_name)) => Ok(CameraSwitcher {
                object: o,
                camera_id,
                camera_name,
                camera_index_name,
            }),
            Err(reason) => Err(FbxTypeMismatch { object: o, reason }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{
        NODE_ATTRIBUTE_CAMERA_SWITCHER_CLASS_NAME, NODE_ATTRIBUTE_TYPE_NAME,
    };
    use fbxscii::LeafAttribute;
    use std::collections::HashMap;

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    fn owned_camera_switcher(attrs: HashMap<String, ElementAttribute>) -> OwnedObject {
        OwnedObject {
            object_index: 42,
            name: "Switcher".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_CAMERA_SWITCHER_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        }
    }

    #[test]
    fn camera_switcher_try_from_ok() {
        let mut attrs = HashMap::new();
        attrs.insert(CAMERA_ID.into(), leaf(&["7"]));
        attrs.insert(CAMERA_NAME.into(), leaf(&["MainCam"]));
        attrs.insert(CAMERA_INDEX_NAME.into(), leaf(&["IndexA"]));
        let o = owned_camera_switcher(attrs);
        let cs = CameraSwitcher::try_from(o).unwrap();
        assert_eq!(cs.camera_id, 7);
        assert_eq!(cs.camera_name, "MainCam");
        assert_eq!(cs.camera_index_name, "IndexA");
    }

    #[test]
    fn camera_switcher_missing_attr() {
        let mut attrs = HashMap::new();
        attrs.insert(CAMERA_ID.into(), leaf(&["1"]));
        attrs.insert(CAMERA_NAME.into(), leaf(&["X"]));
        let o = owned_camera_switcher(attrs);
        let err = CameraSwitcher::try_from(o).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::MissingAttribute {
                name: ref n,
            }
            if n == CAMERA_INDEX_NAME
        ));
    }

    #[test]
    fn camera_switcher_wrong_kind() {
        let o = OwnedObject {
            object_index: 1,
            name: "g".into(),
            type_name: "Geometry".into(),
            class_name: "Mesh".into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let err = CameraSwitcher::try_from(o).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::WrongObjectKind { expected: ref e, ..}
            if e == "CameraSwitcher"
        ));
    }

    #[test]
    fn property_accessors_return_owned_object_properties() {
        let mut attrs = HashMap::new();
        attrs.insert(CAMERA_ID.into(), leaf(&["7"]));
        attrs.insert(CAMERA_NAME.into(), leaf(&["MainCam"]));
        attrs.insert(CAMERA_INDEX_NAME.into(), leaf(&["IndexA"]));

        let mut o = owned_camera_switcher(attrs);
        o.properties
            .insert("SomeFlag".to_string(), Property::Bool(true));

        let cs = CameraSwitcher::try_from(o).unwrap();
        assert_eq!(cs.property("SomeFlag"), Some(&Property::Bool(true)));
        assert_eq!(cs.properties().len(), 1);
    }
}
