//! FBX `Geometry` / `Shape` — Assimp [`ShapeGeometry`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.cpp).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct ShapeGeometry(pub OwnedObject);

impl ShapeGeometry {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for ShapeGeometry {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::ShapeGeometry) => Ok(ShapeGeometry(o)),
            _ => Err(FbxTypeMismatch(o)),
        }
    }
}
