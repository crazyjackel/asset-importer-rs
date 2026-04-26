//! FBX `Deformer` / `Skin` — Assimp [`Skin`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedObject, Property};

use super::{AttrExtractor, FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

const ATTR_LINK_DEFORM_ACURACY: &str = "Link_DeformAcuracy";

#[derive(Debug, PartialEq)]
pub struct Skin {
    object: OwnedObject,
    pub accuracy: f32,
}

impl Skin {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }

    pub fn properties(&self) -> &HashMap<String, Property> {
        &self.object.properties
    }

    pub fn property(&self, name: &str) -> Option<&Property> {
        self.object.properties.get(name)
    }

    pub fn accuracy(&self) -> f32 {
        self.accuracy
    }
}

impl TryFrom<OwnedObject> for Skin {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::Skin) => {
                let accuracy = o
                    .attributes
                    .extract_case_insensitive(ATTR_LINK_DEFORM_ACURACY)
                    .and_then(|a| a.get_tokens().first())
                    .and_then(|t| t.trim().parse::<f32>().ok())
                    .unwrap_or(0.0);
                Ok(Skin { object: o, accuracy })
            }
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "Skin".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use crate::objects::{DEFORMER_SKIN_CLASS_NAME, DEFORMER_TYPE_NAME};
    use crate::{OwnedObject, Property};

    use super::{ATTR_LINK_DEFORM_ACURACY, Skin};

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    #[test]
    fn parses_accuracy_and_properties() {
        let mut attrs = HashMap::new();
        attrs.insert(ATTR_LINK_DEFORM_ACURACY.into(), leaf(&["0.75"]));
        let mut props = HashMap::new();
        props.insert("Foo".into(), Property::Int(1));
        let o = OwnedObject {
            object_index: 10,
            name: "Skin::A".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_SKIN_CLASS_NAME.into(),
            properties: props,
            attributes: attrs,
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let s = Skin::try_from(o).unwrap();
        assert_eq!(s.accuracy(), 0.75);
        assert_eq!(s.property("Foo"), Some(&Property::Int(1)));
    }

    #[test]
    fn defaults_accuracy_to_zero_when_missing() {
        let o = OwnedObject {
            object_index: 11,
            name: "Skin::B".into(),
            type_name: DEFORMER_TYPE_NAME.into(),
            class_name: DEFORMER_SKIN_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        };
        let s = Skin::try_from(o).unwrap();
        assert_eq!(s.accuracy(), 0.0);
    }
}
