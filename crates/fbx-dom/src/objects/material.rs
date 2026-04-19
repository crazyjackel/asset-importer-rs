//! FBX `Material` — Assimp [`Material`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).

use std::collections::HashMap;
use std::convert::TryFrom;

use fbxscii::ElementAttribute;

use crate::OwnedObject;

use super::{
    fbx_object_tag, require_attr_token, FbxObjectTag, FbxTryFromReason, FbxTypeMismatch,
};

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

fn parse_multilayer(attrs: &HashMap<String, ElementAttribute>) -> Result<bool, FbxTryFromReason> {
    let Some(attr) = attrs.get(MULTILAYER) else {
        return Ok(false);
    };
    let tok = attr
        .get_tokens()
        .first()
        .ok_or_else(|| FbxTryFromReason::InvalidAttributeFormat {
            name: MULTILAYER,
            detail: "missing value token".into(),
        })?;
    let v: i32 = tok.parse::<i32>().map_err(|e| FbxTryFromReason::InvalidAttributeFormat {
        name: MULTILAYER,
        detail: e.to_string(),
    })?;
    Ok(v != 0)
}

impl TryFrom<OwnedObject> for Material {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Material) {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Material"));
        }

        let attrs = &o.attributes;
        let shading_raw = match require_attr_token(attrs, SHADING_MODEL) {
            Ok(s) => s,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let shading_model = shading_raw.to_lowercase();
        let multilayer = match parse_multilayer(attrs) {
            Ok(b) => b,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        Ok(Material {
            object: o,
            shading_model,
            multilayer,
        })
    }
}
