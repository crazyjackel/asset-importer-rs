//! FBX `NodeAttribute` / `Null` — Assimp [`Null`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

/// Null / locator node attribute (`NodeAttribute` + class `Null`).
#[derive(Debug, PartialEq)]
pub struct NullNode(pub OwnedObject);

impl NullNode {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for NullNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::NullNode) => Ok(NullNode(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "NullNode")),
        }
    }
}
