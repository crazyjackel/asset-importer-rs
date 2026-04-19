//! FBX `Geometry` / `Line` — Assimp [`LineGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.cpp).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct LineGeometry(pub OwnedObject);

impl LineGeometry {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for LineGeometry {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::LineGeometry) => Ok(LineGeometry(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "LineGeometry".to_string())),
        }
    }
}
