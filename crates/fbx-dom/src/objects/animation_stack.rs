//! FBX `AnimationStack` — Assimp [`AnimationStack`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXAnimation.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).
//!
//! Attached layers are resolved via connections in Assimp; this wrapper exposes the
//! `fbx_simple_property` time fields and the property table on [`OwnedObject`].

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::Property;
use crate::{OwnedDocument, OwnedObject};

use super::{AnimationLayer, FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

#[derive(Debug, PartialEq)]
pub struct AnimationStack(pub OwnedObject);

impl AnimationStack {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }

    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.0.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.0.properties.get(name)
    }

    pub fn local_start(&self) -> i64 {
        match self.property("LocalStart") {
            Some(Property::ILongLong(v)) => *v,
            _ => 0,
        }
    }

    pub fn local_stop(&self) -> i64 {
        match self.property("LocalStop") {
            Some(Property::ILongLong(v)) => *v,
            _ => 0,
        }
    }

    pub fn reference_start(&self) -> i64 {
        match self.property("ReferenceStart") {
            Some(Property::ILongLong(v)) => *v,
            _ => 0,
        }
    }

    pub fn reference_stop(&self) -> i64 {
        match self.property("ReferenceStop") {
            Some(Property::ILongLong(v)) => *v,
            _ => 0,
        }
    }

    /// Resolve incoming `AnimationLayer -> AnimationStack` OO links.
    pub fn get_animation_layers<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> Vec<&'a AnimationLayer> {
        let stack_id = self.inner().object_index;
        document
            .animation_layers
            .iter()
            .filter(|layer| layer.inner().connected_object_ids.contains(&stack_id))
            .collect()
    }
}

impl TryFrom<OwnedObject> for AnimationStack {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::AnimationStack => Ok(AnimationStack(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "AnimationStack".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{
        ANIMATION_LAYER_CLASS_NAME, ANIMATION_LAYER_TYPE_NAME, ANIMATION_STACK_CLASS_NAME,
        ANIMATION_STACK_TYPE_NAME, AnimationLayer,
    };
    use crate::{OwnedDocument, OwnedObject, Property};

    use super::AnimationStack;

    #[test]
    fn stack_time_defaults_and_properties() {
        let o_empty = OwnedObject {
            object_index: 5,
            name: "AnimStack::Take001".into(),
            type_name: ANIMATION_STACK_TYPE_NAME.into(),
            class_name: ANIMATION_STACK_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let s = AnimationStack::try_from(o_empty).unwrap();
        assert_eq!(s.local_start(), 0);
        assert_eq!(s.local_stop(), 0);
        assert_eq!(s.reference_start(), 0);
        assert_eq!(s.reference_stop(), 0);

        let mut properties = HashMap::new();
        properties.insert("LocalStart".to_string(), Property::ILongLong(10));
        properties.insert("LocalStop".to_string(), Property::ILongLong(100));
        properties.insert("ReferenceStart".to_string(), Property::ILongLong(20));
        properties.insert("ReferenceStop".to_string(), Property::ILongLong(200));
        let o = OwnedObject {
            object_index: 6,
            name: "AnimStack::Take002".into(),
            type_name: ANIMATION_STACK_TYPE_NAME.into(),
            class_name: ANIMATION_STACK_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let s = AnimationStack::try_from(o).unwrap();
        assert_eq!(s.local_start(), 10);
        assert_eq!(s.local_stop(), 100);
        assert_eq!(s.reference_start(), 20);
        assert_eq!(s.reference_stop(), 200);
    }

    #[test]
    fn resolves_animation_layer_links() {
        let stack = AnimationStack::try_from(OwnedObject {
            object_index: 1000,
            name: "AnimStack::Main".into(),
            type_name: ANIMATION_STACK_TYPE_NAME.into(),
            class_name: ANIMATION_STACK_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let layer = AnimationLayer::try_from(OwnedObject {
            object_index: 1001,
            name: "AnimLayer::Layer0".into(),
            type_name: ANIMATION_LAYER_TYPE_NAME.into(),
            class_name: ANIMATION_LAYER_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![1000],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let mut owned = OwnedDocument::default();
        owned.animation_layers = vec![layer];
        let links = stack.get_animation_layers(&owned);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].inner().object_index, 1001);
    }
}
