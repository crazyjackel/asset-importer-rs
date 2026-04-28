//! FBX `Deformer` / `BlendShape` — Assimp [`BlendShape`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedDocument, OwnedObject, Property};

use super::{BlendShapeChannel, FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

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

    /// Resolve `BlendShapeChannel -> BlendShape` links via owned OO connections.
    pub fn get_blend_shape_channels<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> Vec<&'a BlendShapeChannel> {
        let blend_shape_id = self.inner().object_index;
        document
            .blend_shape_channels
            .iter()
            .filter(|channel| channel.inner().connected_object_ids.contains(&blend_shape_id))
            .collect()
    }
}

impl TryFrom<OwnedObject> for BlendShape {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::BlendShape => Ok(BlendShape(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "BlendShape".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::objects::{
        BlendShapeChannel, DEFORMER_BLEND_SHAPE_CHANNEL_CLASS_NAME, DEFORMER_BLEND_SHAPE_CLASS_NAME,
        DEFORMER_TYPE_NAME,
    };
    use crate::{OwnedDocument, OwnedObject, Property};

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

    #[test]
    fn resolves_blend_shape_channel_connections() {
        let blend_shape = BlendShape::try_from(OwnedObject {
            object_index: 50,
            name: "BlendShape::B".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_BLEND_SHAPE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let channel = BlendShapeChannel::try_from(OwnedObject {
            object_index: 51,
            name: "BlendShapeChannel::B".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_BLEND_SHAPE_CHANNEL_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![50],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut owned = OwnedDocument::default();
        owned.blend_shape_channels = vec![channel];
        let linked = blend_shape.get_blend_shape_channels(&owned);
        assert_eq!(linked.len(), 1);
        assert_eq!(linked[0].inner().object_index, 51);
    }
}
