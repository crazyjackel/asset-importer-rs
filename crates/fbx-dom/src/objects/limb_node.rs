//! FBX `NodeAttribute` / `LimbNode` — Assimp [`LimbNode`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct LimbNode(pub OwnedObject);

impl LimbNode {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for LimbNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::LimbNode) => Ok(LimbNode(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "LimbNode".to_string())),
        }
    }
}
