//! FBX `Texture` — Assimp [`Texture`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::Property;
use crate::objects::AttrExtractorExt;
use crate::objects::AttrExtractorParseExt;
use crate::{OwnedDocument, OwnedObject};

use super::{FbxObjectTag, FbxTypeMismatch, Video, fbx_object_tag};

const TYPE_ATTR: &str = "Type";
const FILE_NAME_ATTR: &str = "FileName";
const RELATIVE_FILENAME_ATTR: &str = "RelativeFilename";
const MODEL_UV_TRANSLATION: &str = "ModelUVTranslation";
const MODEL_UV_SCALING: &str = "ModelUVScaling";
const TEXTURE_ALPHA_SOURCE: &str = "Texture_Alpha_Source";
const CROPPING: &str = "Cropping";

const PROP_SCALING: &str = "Scaling";
const PROP_TRANSLATION: &str = "Translation";
const PROP_ROTATION: &str = "Rotation";

#[derive(Debug, PartialEq)]
pub struct Texture {
    object: OwnedObject,
    pub texture_type: String,
    pub file_name: String,
    pub relative_file_name: Option<String>,
    pub uv_translation: [f32; 2],
    pub uv_scaling: [f32; 2],
    pub uv_rotation: f32,
    pub cropping: [i32; 4],
    pub alpha_source: String,
}

impl Texture {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }

    /// Resolve incoming `Video -> Texture` OO links (Assimp `Texture::Media`).
    pub fn get_media_video<'a>(&'a self, document: &'a OwnedDocument) -> Option<&'a Video> {
        let texture_id = self.inner().object_index;
        document
            .videos
            .iter()
            .find(|video| video.inner().connected_object_ids.contains(&texture_id))
    }
}

fn apply_texture_property_uv_overrides(
    properties: &HashMap<String, Property>,
    uv_translation: &mut [f32; 2],
    uv_scaling: &mut [f32; 2],
    uv_rotation: &mut f32,
) {
    // 3ds Max / FBX SDK path (Assimp reads after element scope).
    if let Some(Property::Vec3(v)) = properties.get(PROP_SCALING) {
        uv_scaling[0] = v[0];
        uv_scaling[1] = v[1];
    }
    if let Some(Property::Vec3(v)) = properties.get(PROP_TRANSLATION) {
        uv_translation[0] = v[0];
        uv_translation[1] = v[1];
    }
    if let Some(Property::Vec3(v)) = properties.get(PROP_ROTATION) {
        *uv_rotation = v[2];
    }
}

impl TryFrom<OwnedObject> for Texture {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != FbxObjectTag::Texture {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Texture".to_string()));
        }

        let attrs = &o.attributes;
        let props = &o.properties;

        let texture_type = match attrs.optional_token_case_insensitive(&TYPE_ATTR) {
            Ok(s) => s.map(|s| s.to_string()).unwrap_or_default(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let file_name = match attrs.optional_token_case_insensitive(&FILE_NAME_ATTR) {
            Ok(s) => s.map(|s| s.to_string()).unwrap_or_default(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let relative_file_name =
            match attrs.optional_token_case_insensitive(&RELATIVE_FILENAME_ATTR) {
                Ok(r) => r.map(|s| s.to_string()),
                Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
            };

        let mut uv_translation = [0.0f32, 0.0f32];
        let mut uv_scaling = [1.0f32, 1.0f32];
        let mut uv_rotation = 0.0f32;
        let mut cropping = [0i32; 4];

        match attrs.optional_two_f32(&MODEL_UV_TRANSLATION) {
            Ok(Some(t)) => uv_translation = t,
            Ok(None) => {}
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        }
        match attrs.optional_two_f32_case_insensitive(&MODEL_UV_SCALING) {
            Ok(Some(t)) => uv_scaling = t,
            Ok(None) => {}
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        }
        match attrs.optional_four_i32_case_insensitive(&CROPPING) {
            Ok(Some(c)) => cropping = c,
            Ok(None) => {}
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        }

        apply_texture_property_uv_overrides(
            props,
            &mut uv_translation,
            &mut uv_scaling,
            &mut uv_rotation,
        );

        let alpha_source = match attrs.optional_token_case_insensitive(&TEXTURE_ALPHA_SOURCE) {
            Ok(s) => s.map(|s| s.to_string()).unwrap_or_default(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        Ok(Texture {
            object: o,
            texture_type,
            file_name,
            relative_file_name,
            uv_translation,
            uv_scaling,
            uv_rotation,
            cropping,
            alpha_source,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use super::Texture;
    use crate::OwnedDocument;
    use crate::objects::{
        TEXTURE_CLASS_NAME, TEXTURE_TYPE_NAME, VIDEO_CLASS_NAME, VIDEO_TYPE_NAME, Video,
    };

    #[test]
    fn resolves_media_video_connection() {
        let texture = Texture::try_from(crate::OwnedObject {
            object_index: 700,
            name: "Texture::A".into(),
            type_name: TEXTURE_TYPE_NAME.into(),
            class_name: TEXTURE_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::new(),
            connected_object_ids: vec![],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let video = Video::try_from(crate::OwnedObject {
            object_index: 701,
            name: "Video::A".into(),
            type_name: VIDEO_TYPE_NAME.into(),
            class_name: VIDEO_CLASS_NAME.into(),
            properties: HashMap::new(),
            attributes: HashMap::from([
                (
                    "Type".to_string(),
                    fbxscii::ElementAttribute::Leaf(Box::new(fbxscii::LeafAttribute {
                        key: "Type".into(),
                        tokens: vec!["Clip".into()],
                    })),
                ),
                (
                    "FileName".to_string(),
                    fbxscii::ElementAttribute::Leaf(Box::new(fbxscii::LeafAttribute {
                        key: "FileName".into(),
                        tokens: vec!["a.png".into()],
                    })),
                ),
            ]),
            connected_object_ids: vec![700],
            object_property_targets: vec![],
            pp_property_targets: HashMap::new(),
        })
        .unwrap();
        let mut doc = OwnedDocument::default();
        doc.videos = vec![video];
        assert_eq!(
            texture
                .get_media_video(&doc)
                .map(|v| v.inner().object_index),
            Some(701)
        );
    }
}
