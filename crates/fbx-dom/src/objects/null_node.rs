//! FBX `NodeAttribute` / `Null` — Assimp [`Null`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

/// Null / locator node attribute (`NodeAttribute` + class `Null`).
#[derive(Debug, PartialEq)]
pub struct NullNode(pub OwnedObject);

impl NullNode {
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
}

impl TryFrom<OwnedObject> for NullNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::NullNode => Ok(NullNode(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "NullNode".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{NODE_ATTRIBUTE_NULL_CLASS_NAME, NODE_ATTRIBUTE_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::NullNode;

    #[test]
    fn property_accessors_return_owned_object_properties() {
        let mut properties = HashMap::new();
        properties.insert("Size".to_string(), Property::Float(100.0));
        let o = OwnedObject {
            object_index: 99,
            name: "Null".into(),
            type_name: NODE_ATTRIBUTE_TYPE_NAME.into(),
            class_name: NODE_ATTRIBUTE_NULL_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let null_node = NullNode::try_from(o).unwrap();
        assert_eq!(null_node.property("Size"), Some(&Property::Float(100.0)));
        assert_eq!(null_node.properties().len(), 1);
    }
}
