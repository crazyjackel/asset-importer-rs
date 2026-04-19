//! FBX `Texture` — Assimp [`Texture`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct Texture(pub OwnedObject);

impl Texture {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for Texture {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::Texture) => Ok(Texture(o)),
            _ => Err(FbxTypeMismatch(o)),
        }
    }
}
