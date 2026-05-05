//! FBX `NodeAttribute` / `LimbNode` — Assimp [`LimbNode`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

const PROP_LIMB_LENGTH: &str = "LimbLength";
const PROP_SIZE: &str = "Size";

#[derive(Debug, PartialEq)]
pub struct LimbNode(pub OwnedObject);

impl LimbNode {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }

    /// Temporary bridge to Assimp-style node-attribute properties until typed accessors are added.
    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.0.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.0.properties.get(name)
    }

    /// Common skeleton length property used by some exporters.
    pub fn limb_length(&self) -> f32 {
        match self.property(PROP_LIMB_LENGTH) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }

    /// Marker/display size value when present on limb-node properties.
    pub fn size(&self) -> f32 {
        match self.property(PROP_SIZE) {
            Some(Property::Float(v)) => *v,
            _ => 0.0,
        }
    }
}

impl TryFrom<OwnedObject> for LimbNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::LimbNode => Ok(LimbNode(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "LimbNode".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{NODE_ATTRIBUTE_LIMB_NODE_CLASS_NAME, NODE_ATTRIBUTE_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::LimbNode;

    #[test]
    fn property_accessors_return_owned_object_properties() {
        let mut properties = HashMap::new();
        properties.insert("LimbLength".to_string(), Property::Float(10.0));
        let o = OwnedObject {
            object_index: 99,
            name: "Limb".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIMB_NODE_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let limb_node = LimbNode::try_from(o).unwrap();
        assert_eq!(
            limb_node.property("LimbLength"),
            Some(&Property::Float(10.0))
        );
        assert_eq!(limb_node.properties().len(), 1);
    }

    #[test]
    fn typed_accessors_return_defaults_and_values() {
        let o_default = OwnedObject {
            object_index: 99,
            name: "Limb".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIMB_NODE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let limb_default = LimbNode::try_from(o_default).unwrap();
        assert_eq!(limb_default.limb_length(), 0.0);
        assert_eq!(limb_default.size(), 0.0);

        let mut properties = HashMap::new();
        properties.insert("LimbLength".to_string(), Property::Float(8.5));
        properties.insert("Size".to_string(), Property::Float(2.0));
        let o_values = OwnedObject {
            object_index: 100,
            name: "Limb".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_LIMB_NODE_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let limb_values = LimbNode::try_from(o_values).unwrap();
        assert_eq!(limb_values.limb_length(), 8.5);
        assert_eq!(limb_values.size(), 2.0);
    }
}
