//! FBX `Deformer` / `BlendShape` — Assimp [`BlendShape`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct BlendShape(pub OwnedObject);

impl BlendShape {
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

impl TryFrom<OwnedObject> for BlendShape {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::BlendShape) => Ok(BlendShape(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "BlendShape".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{DEFORMER_BLEND_SHAPE_CLASS_NAME, DEFORMER_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::BlendShape;

    #[test]
    fn property_accessors() {
        let mut properties = HashMap::new();
        properties.insert("Foo".into(), Property::Int(7));
        let o = OwnedObject {
            object_index: 15,
            name: "BlendShape::A".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_BLEND_SHAPE_CLASS_NAME.into(),
            properties,
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let b = BlendShape::try_from(o).unwrap();
        assert_eq!(b.property("Foo"), Some(&Property::Int(7)));
    }
}
