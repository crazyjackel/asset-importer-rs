//! FBX `Model` objects — Assimp [`Model`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

/// Typed wrapper for a scene graph model / transform node (`Model::*` except unsupported effectors).
#[derive(Debug, PartialEq)]
pub struct Model(pub OwnedObject);

impl Model {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for Model {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::Model) => Ok(Model(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "Model")),
        }
    }
}
