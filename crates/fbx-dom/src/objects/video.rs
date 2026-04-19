//! FBX `Video` — Assimp [`Video`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{
    fbx_object_tag, optional_attr_tokens_case_insensitive, optional_nonempty_string_case_insensitive,
    require_attr_token, require_attr_token_case_insensitive, FbxObjectTag, FbxTypeMismatch,
};

const TYPE_ATTR: &str = "Type";
const FILE_NAME_ATTR: &str = "FileName";
const RELATIVE_FILENAME_ATTR: &str = "RelativeFilename";
const CONTENT_ATTR: &str = "Content";

#[derive(Debug, PartialEq)]
pub struct Video {
    object: OwnedObject,
    pub video_type: String,
    pub file_name: String,
    pub relative_file_name: Option<String>,
    /// Raw `Content` value tokens (ASCII: often base64-quoted chunks; Assimp decodes these separately).
    pub content: Option<Vec<String>>,
}

impl Video {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

impl TryFrom<OwnedObject> for Video {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Video) {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Video"));
        }

        let attrs = &o.attributes;
        let video_type = match require_attr_token(attrs, TYPE_ATTR) {
            Ok(s) => s.to_string(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        let file_name = match require_attr_token_case_insensitive(attrs, FILE_NAME_ATTR) {
            Ok(s) => s.to_string(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let relative_file_name =
            match optional_nonempty_string_case_insensitive(attrs, RELATIVE_FILENAME_ATTR) {
                Ok(r) => r,
                Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
            };
        let content = match optional_attr_tokens_case_insensitive(attrs, CONTENT_ATTR) {
            Ok(c) => c,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        Ok(Video {
            object: o,
            video_type,
            file_name,
            relative_file_name,
            content,
        })
    }
}
