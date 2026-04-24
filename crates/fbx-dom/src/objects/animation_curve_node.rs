//! FBX `AnimationCurveNode` — Assimp [`AnimationCurveNode`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXAnimation.cpp) / [`FBXDocument.h`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).
//!
//! Target object and animated property name are resolved via connections in Assimp; this wrapper
//! only exposes the property table on [`OwnedObject`].

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::OwnedObject;
use crate::Property;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct AnimationCurveNode(pub OwnedObject);

impl AnimationCurveNode {
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
}

impl TryFrom<OwnedObject> for AnimationCurveNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::AnimationCurveNode) => Ok(AnimationCurveNode(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "AnimationCurveNode".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{
        ANIMATION_CURVE_NODE_CLASS_NAME, ANIMATION_CURVE_NODE_TYPE_NAME,
    };
    use crate::{OwnedObject, Property};

    use super::AnimationCurveNode;

    #[test]
    fn property_accessors() {
        let mut properties = HashMap::new();
        properties.insert("d|Visibility".to_string(), Property::Float(1.0));
        let o = OwnedObject {
            object_index: 2,
            name: "AnimCurveNode::T".into(),
            type_name: ANIMATION_CURVE_NODE_TYPE_NAME.into(),
            class_name: ANIMATION_CURVE_NODE_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let n = AnimationCurveNode::try_from(o).unwrap();
        assert_eq!(n.property("d|Visibility"), Some(&Property::Float(1.0)));
    }
}
