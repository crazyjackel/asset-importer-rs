//! FBX `Texture` — Assimp [`Texture`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{
    fbx_object_tag, optional_nonempty_string_case_insensitive, require_attr_token,
    require_attr_token_case_insensitive, FbxObjectTag, FbxTypeMismatch,
};

const TYPE_ATTR: &str = "Type";
const FILE_NAME_ATTR: &str = "FileName";
const RELATIVE_FILENAME_ATTR: &str = "RelativeFilename";

#[derive(Debug, PartialEq)]
pub struct Texture {
    object: OwnedObject,
    pub texture_type: String,
    pub file_name: String,
    pub relative_file_name: Option<String>,
}

impl Texture {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

impl TryFrom<OwnedObject> for Texture {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Texture) {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Texture"));
        }

        let attrs = &o.attributes;
        let texture_type = match require_attr_token(attrs, TYPE_ATTR) {
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

        Ok(Texture {
            object: o,
            texture_type,
            file_name,
            relative_file_name,
        })
    }
}
