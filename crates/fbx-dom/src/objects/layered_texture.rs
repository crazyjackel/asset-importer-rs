//! FBX `LayeredTexture` — Assimp [`LayeredTexture`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::{OwnedDocument, OwnedObject};

use super::{AttrExtractorExt, FbxObjectTag, FbxTypeMismatch, Texture, fbx_object_tag};

const BLEND_MODES_ATTR: &str = "BlendModes";
const ALPHAS_ATTR: &str = "Alphas";

#[derive(Debug, PartialEq)]
pub struct LayeredTexture {
    object: OwnedObject,
    pub blend_mode: i32,
    pub alpha: f32,
}

impl LayeredTexture {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }

    pub fn blend_mode(&self) -> i32 {
        self.blend_mode
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Resolve incoming `Texture -> LayeredTexture` links.
    pub fn get_textures<'a>(&'a self, document: &'a OwnedDocument) -> Vec<&'a Texture> {
        let layered_texture_id = self.inner().object_index;
        document
            .textures
            .iter()
            .filter(|texture| {
                texture
                    .inner()
                    .connected_object_ids
                    .contains(&layered_texture_id)
            })
            .collect()
    }

    pub fn texture_count(&self, document: &OwnedDocument) -> usize {
        self.get_textures(document).len()
    }
}

impl TryFrom<OwnedObject> for LayeredTexture {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::LayeredTexture => {
                let blend_mode = match o
                    .attributes
                    .optional_token_case_insensitive(BLEND_MODES_ATTR)
                {
                    Ok(v) => v.and_then(|x| x.trim().parse::<i32>().ok()).unwrap_or(0),
                    Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
                };
                let alpha = match o.attributes.optional_token_case_insensitive(ALPHAS_ATTR) {
                    Ok(v) => v.and_then(|x| x.trim().parse::<f32>().ok()).unwrap_or(1.0),
                    Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
                };
                Ok(LayeredTexture {
                    object: o,
                    blend_mode,
                    alpha,
                })
            }
            _ => Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "LayeredTexture".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use fbxscii::{ElementAttribute, LeafAttribute};

    use super::*;
    use crate::Property;
    use crate::objects::{
        LAYERED_TEXTURE_CLASS_NAME, LAYERED_TEXTURE_TYPE_NAME, TEXTURE_CLASS_NAME,
        TEXTURE_TYPE_NAME,
    };

    fn leaf(tokens: &[&str]) -> ElementAttribute {
        ElementAttribute::Leaf(Box::new(LeafAttribute {
            key: String::new(),
            tokens: tokens.iter().map(|s| (*s).to_string()).collect(),
        }))
    }

    #[test]
    fn parses_blend_mode_and_alpha() {
        let layered = LayeredTexture::try_from(OwnedObject {
            object_index: 900,
            name: "LayeredTexture::A".into(),
            type_name: LAYERED_TEXTURE_TYPE_NAME.into(),
            class_name: LAYERED_TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::from([
                (BLEND_MODES_ATTR.to_string(), leaf(&["1"])),
                (ALPHAS_ATTR.to_string(), leaf(&["0.25"])),
            ]),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        assert_eq!(layered.blend_mode(), 1);
        assert_eq!(layered.alpha(), 0.25);
    }

    #[test]
    fn defaults_blend_mode_and_alpha() {
        let layered = LayeredTexture::try_from(OwnedObject {
            object_index: 901,
            name: "LayeredTexture::B".into(),
            type_name: LAYERED_TEXTURE_TYPE_NAME.into(),
            class_name: LAYERED_TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        assert_eq!(layered.blend_mode(), 0);
        assert_eq!(layered.alpha(), 1.0);
    }

    #[test]
    fn resolves_textures_and_count() {
        let layered = LayeredTexture::try_from(OwnedObject {
            object_index: 902,
            name: "LayeredTexture::C".into(),
            type_name: LAYERED_TEXTURE_TYPE_NAME.into(),
            class_name: LAYERED_TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let texture = Texture::try_from(OwnedObject {
            object_index: 903,
            name: "Texture::A".into(),
            type_name: TEXTURE_TYPE_NAME.into(),
            class_name: TEXTURE_CLASS_NAME.into(),
            properties: HashMap::from([("Foo".into(), Property::Int(1))]),
            attributes: HashMap::new(),
            connected_object_ids: vec![902],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let mut owned = OwnedDocument::default();
        owned.textures = vec![texture];
        let links = layered.get_textures(&owned);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].inner().object_index, 903);
        assert_eq!(layered.texture_count(&owned), 1);
    }
}
