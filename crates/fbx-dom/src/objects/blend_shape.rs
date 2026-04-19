//! FBX `Deformer` / `BlendShape` — Assimp [`BlendShape`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct BlendShape(pub OwnedObject);

impl BlendShape {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for BlendShape {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::BlendShape) => Ok(BlendShape(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "BlendShape")),
        }
    }
}
