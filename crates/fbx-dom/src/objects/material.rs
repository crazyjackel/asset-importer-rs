//! FBX `Material` — Assimp [`Material`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{OwnedDocument, OwnedObject, Property, objects::AttrExtractorExt};

use super::{FbxObjectTag, FbxTypeMismatch, LayeredTexture, Texture, fbx_object_tag};

const SHADING_MODEL: &str = "ShadingModel";
const MULTILAYER: &str = "MultiLayer";

#[derive(Debug, PartialEq)]
pub struct Material {
    pub object: OwnedObject,
    pub shading_model: String,
    pub multilayer: bool,
}

impl Material {
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

    /// Resolve incoming `Texture -> Material` OP links keyed by destination property name.
    pub fn get_textures<'a>(&'a self, document: &'a OwnedDocument) -> HashMap<&'a str, &'a Texture> {
        let material_id = self.inner().object_index;
        let mut out = HashMap::new();
        for texture in &document.textures {
            for conn in &texture.inner().object_property_targets {
                if conn.dest == material_id && !conn.property.is_empty() {
                    out.insert(conn.property.as_str(), texture);
                }
            }
        }
        out
    }

    /// Resolve incoming `LayeredTexture -> Material` OP links keyed by destination property name.
    pub fn get_layered_textures<'a>(
        &'a self,
        document: &'a OwnedDocument,
    ) -> HashMap<&'a str, &'a LayeredTexture> {
        let material_id = self.inner().object_index;
        let mut out = HashMap::new();
        for layered in &document.layered_textures {
            for conn in &layered.inner().object_property_targets {
                if conn.dest == material_id && !conn.property.is_empty() {
                    out.insert(conn.property.as_str(), layered);
                }
            }
        }
        out
    }
}

impl TryFrom<OwnedObject> for Material {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != FbxObjectTag::Material {
            return Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "Material".to_string(),
            ));
        }

        let attrs = &o.attributes;
        let shading_raw = match attrs.require_token(&SHADING_MODEL) {
            Ok(s) => s,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let shading_model = shading_raw.to_lowercase();
        let multilayer = match attrs
            .require_token(MULTILAYER)
            .map(|t| t.parse::<i32>().unwrap_or(0))
        {
            Ok(b) => b != 0,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        Ok(Material {
            object: o,
            shading_model,
            multilayer,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::ObjectPropertyConnection;
    use crate::objects::{
        LAYERED_TEXTURE_CLASS_NAME, LAYERED_TEXTURE_TYPE_NAME, MATERIAL_CLASS_NAME, MATERIAL_TYPE_NAME, TEXTURE_CLASS_NAME,
        TEXTURE_TYPE_NAME,
    };
    use crate::{OwnedDocument, OwnedObject};

    use super::Material;

    #[test]
    fn resolves_texture_and_layered_texture_links() {
        let material = Material::try_from(OwnedObject {
            object_index: 800,
            name: "Material::M".into(),
            type_name: MATERIAL_TYPE_NAME.into(),
            class_name: MATERIAL_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::from([
                (
                    "ShadingModel".to_string(),
                    fbxscii::ElementAttribute::Leaf(Box::new(fbxscii::LeafAttribute {
                        key: "ShadingModel".into(),
                        tokens: vec!["Phong".into()],
                    })),
                ),
                (
                    "MultiLayer".to_string(),
                    fbxscii::ElementAttribute::Leaf(Box::new(fbxscii::LeafAttribute {
                        key: "MultiLayer".into(),
                        tokens: vec!["0".into()],
                    })),
                ),
            ]),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let texture = crate::objects::Texture::try_from(OwnedObject {
            object_index: 801,
            name: "Texture::A".into(),
            type_name: TEXTURE_TYPE_NAME.into(),
            class_name: TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 800,
                property: "DiffuseColor".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let layered = crate::objects::LayeredTexture::try_from(OwnedObject {
            object_index: 802,
            name: "LayeredTexture::A".into(),
            type_name: LAYERED_TEXTURE_TYPE_NAME.into(),
            class_name: LAYERED_TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![ObjectPropertyConnection {
                dest: 800,
                property: "SpecularColor".into(),
            }],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();

        let mut owned = OwnedDocument::default();
        owned.textures = vec![texture];
        owned.layered_textures = vec![layered];

        let textures = material.get_textures(&owned);
        assert_eq!(textures.len(), 1);
        assert_eq!(textures.get("DiffuseColor").map(|t| t.inner().object_index), Some(801));

        let layered_textures = material.get_layered_textures(&owned);
        assert_eq!(layered_textures.len(), 1);
        assert_eq!(
            layered_textures
                .get("SpecularColor")
                .map(|t| t.inner().object_index),
            Some(802)
        );
    }
}
