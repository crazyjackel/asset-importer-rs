//! FBX `AnimationCurveNode` — Assimp [`AnimationCurveNode`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h).

use std::convert::TryFrom;

use crate::OwnedObject;

use super::{fbx_object_tag, FbxObjectTag, FbxTypeMismatch};

#[derive(Debug, PartialEq)]
pub struct AnimationCurveNode(pub OwnedObject);

impl AnimationCurveNode {
    pub fn inner(&self) -> &OwnedObject {
        &self.0
    }

    pub fn into_inner(self) -> OwnedObject {
        self.0
    }
}

impl TryFrom<OwnedObject> for AnimationCurveNode {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            Some(FbxObjectTag::AnimationCurveNode) => Ok(AnimationCurveNode(o)),
            _ => Err(FbxTypeMismatch(o)),
        }
    }
}
