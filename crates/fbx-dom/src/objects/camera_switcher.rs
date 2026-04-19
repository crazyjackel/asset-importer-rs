//! FBX `NodeAttribute` / `CameraSwitcher` — Assimp [`CameraSwitcher`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct CameraSwitcher(pub OwnedObject);

impl CameraSwitcher {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for CameraSwitcher {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::CameraSwitcher) => Ok(CameraSwitcher(o)),
            _ => Err(FbxTypeMismatch(o)),
        }
    }
}
