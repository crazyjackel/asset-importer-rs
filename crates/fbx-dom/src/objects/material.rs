//! FBX `Material` — Assimp [`Material`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).

use std::convert::TryFrom;

use crate::{OwnedObject, objects::AttrExtractorExt};

use super::{FbxObjectTag, FbxTypeMismatch, fbx_object_tag};

const SHADING_MODEL: &str = "ShadingModel";
const MULTILAYER: &str = "MultiLayer";

#[derive(Debug, PartialEq)]
pub struct Material {
    object: OwnedObject,
    /// Lowercased like Assimp’s material ctor (for template / shading comparisons).
    pub shading_model: String,
    pub multilayer: bool,
}

impl Material {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

impl TryFrom<OwnedObject> for Material {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Material) {
            return Err(FbxTypeMismatch::wrong_object_kind(
                o,
                "Material".to_string(),
            ));
        }

        let attrs = &o.attributes;
        let shading_raw = match attrs.require_token(&SHADING_MODEL) {
            Ok(s) => s,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let shading_model = shading_raw.to_lowercase();
        let multilayer = match attrs
            .require_token(MULTILAYER)
            .map(|t| t.parse::<i32>().unwrap_or(0))
        {
            Ok(b) => b != 0,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        Ok(Material {
            object: o,
            shading_model,
            multilayer,
        })
    }
}
