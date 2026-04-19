//! FBX `Geometry` / `Mesh` — Assimp [`MeshGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.cpp).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct MeshGeometry(pub OwnedObject);

impl MeshGeometry {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for MeshGeometry {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::MeshGeometry) => Ok(MeshGeometry(o)),
            _ => Err(FbxTypeMismatch::wrong_object_kind(o, "MeshGeometry".to_string())),
        }
    }
}
