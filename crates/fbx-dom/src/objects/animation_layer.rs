//! FBX `AnimationLayer` — Assimp [`AnimationLayer`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXAnimation.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).
//!
//! Attached curve nodes are resolved via connections in Assimp; this wrapper exposes the
//! `AnimationLayer.FbxAnimLayer` property table on [`OwnedObject`].

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::OwnedObject;
use crate::Property;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct AnimationLayer(pub OwnedObject);

impl AnimationLayer {
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

    /// FBX SDK / template `Weight` (percentage); Assimp treats the layer property table as optional.
    pub fn weight(&self) -> f32 {
        match self.property("Weight") {
            Some(Property::Float(v)) => *v,
            _ => 100.0,
        }
    }

    pub fn mute(&self) -> bool {
        match self.property("Mute") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }

    pub fn solo(&self) -> bool {
        match self.property("Solo") {
            Some(Property::Bool(v)) => *v,
            _ => false,
        }
    }
}

impl TryFrom<OwnedObject> for AnimationLayer {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::AnimationLayer) => Ok(AnimationLayer(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "AnimationLayer".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{ANIMATION_LAYER_CLASS_NAME, ANIMATION_LAYER_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::AnimationLayer;

    #[test]
    fn layer_weight_mute_solo_defaults_and_properties() {
        let o_empty = OwnedObject {
            object_index: 3,
            name: "AnimLayer::L0".into(),
            type_name: ANIMATION_LAYER_TYPE_NAME.into(),
            class_name: ANIMATION_LAYER_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let layer = AnimationLayer::try_from(o_empty).unwrap();
        assert_eq!(layer.weight(), 100.0);
        assert_eq!(layer.mute(), false);
        assert_eq!(layer.solo(), false);

        let mut properties = HashMap::new();
        properties.insert("Weight".to_string(), Property::Float(50.0));
        properties.insert("Mute".to_string(), Property::Bool(true));
        properties.insert("Solo".to_string(), Property::Bool(true));
        let o = OwnedObject {
            object_index: 4,
            name: "AnimLayer::L1".into(),
            type_name: ANIMATION_LAYER_TYPE_NAME.into(),
            class_name: ANIMATION_LAYER_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let layer = AnimationLayer::try_from(o).unwrap();
        assert_eq!(layer.weight(), 50.0);
        assert_eq!(layer.mute(), true);
        assert_eq!(layer.solo(), true);
    }
}
