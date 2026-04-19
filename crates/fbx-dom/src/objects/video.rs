//! FBX `Video` — Assimp [`Video`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXMaterial.cpp).
//!
//! ASCII `Content` is optional base64 (often several value tokens); we decode each token and
//! concatenate the bytes. Assimp’s binary `Content` (`R` + length + raw bytes) is not handled here.

use std::collections::HashMap;
use std::convert::TryFrom;

use fbxscii::ElementAttribute;

use crate::OwnedObject;

use super::{
    FbxObjectTag, FbxTryFromReason, FbxTypeMismatch, fbx_object_tag,
    optional_attr_tokens_case_insensitive, optional_nonempty_string_case_insensitive,
    require_attr_token_case_insensitive,
};

const TYPE_ATTR: &str = "Type";
const FILE_NAME_ATTR: &str = "FileName";
const RELATIVE_FILENAME_ATTR: &str = "RelativeFilename";
const CONTENT_ATTR: &str = "Content";

#[derive(Debug, PartialEq)]
pub struct Video {
    object: OwnedObject,
    /// `Type` child (case-insensitive key); e.g. clip vs embedded video metadata from the exporter.
    pub video_type: String,
    /// `FileName` / `Filename` (case-insensitive), per Assimp `FindElementCaseInsensitive` for video.
    pub file_name: String,
    /// `RelativeFilename` when present (case-insensitive key).
    pub relative_file_name: Option<String>,
    /// Decoded `Content` when present: each DOM token is its own base64 payload; outputs are concatenated.
    pub content: Option<Vec<u8>>,
}

impl Video {
    pub fn inner(&self) -> &OwnedObject {
        &self.object
    }

    pub fn into_inner(self) -> OwnedObject {
        self.object
    }
}

/// Parse optional `Content`: missing attribute, missing tokens, or all-empty tokens → `None`.
///
/// FBX may split large embedded payloads across **multiple** value tokens. Each token is a
/// separate base64 string (Assimp decodes per token then appends); you cannot decode the
/// concatenation of raw token text as one base64 stream.
fn decode_optional_content(
    attrs: &HashMap<String, ElementAttribute>,
) -> Result<Option<Vec<u8>>, FbxTryFromReason> {
    let Some(tokens) = optional_attr_tokens_case_insensitive(attrs, CONTENT_ATTR)? else {
        return Ok(None);
    };
    if tokens.is_empty() {
        return Ok(None);
    }

    // One decode per token, then concat — same semantics as Assimp’s two-pass size+decode loop.
    let mut out = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        let s = t.trim();
        // ASCII FBX often wraps each chunk in double quotes (Assimp strips them).
        let payload = if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
            &s[1..s.len() - 1]
        } else {
            s
        };
        let decoded =
            base64::decode(payload).map_err(|e| FbxTryFromReason::InvalidAttributeFormat {
                name: CONTENT_ATTR,
                detail: format!("base64 decode (token {i}): {e}"),
            })?;
        if decoded.is_empty() {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: CONTENT_ATTR,
                detail: format!("base64 token {i} decoded to empty"),
            });
        }
        out.extend_from_slice(&decoded);
    }

    Ok(Some(out))
}

impl TryFrom<OwnedObject> for Video {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        if fbx_object_tag(&o) != Some(FbxObjectTag::Video) {
            return Err(FbxTypeMismatch::wrong_object_kind(o, "Video"));
        }

        let attrs = &o.attributes;
        // Case-insensitive keys match common exporter spelling drift (e.g. `Filename` vs `FileName`).
        let video_type = match require_attr_token_case_insensitive(attrs, TYPE_ATTR) {
            Ok(s) => s.to_string(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        let file_name = match require_attr_token_case_insensitive(attrs, FILE_NAME_ATTR) {
            Ok(s) => s.to_string(),
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };
        let relative_file_name =
            match optional_nonempty_string_case_insensitive(attrs, RELATIVE_FILENAME_ATTR) {
                Ok(r) => r,
                Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
            };
        let content = match decode_optional_content(attrs) {
            Ok(c) => c,
            Err(reason) => return Err(FbxTypeMismatch { object: o, reason }),
        };

        Ok(Video {
            object: o,
            video_type,
            file_name,
            relative_file_name,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use fbxscii::{ElementAttribute, LeafAttribute};

    use super::*;

    #[test]
    fn decode_content_quoted_base64() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "Content".to_string(),
            ElementAttribute::Leaf(Box::new(LeafAttribute {
                key: "Content".into(),
                tokens: vec!["\"aGVsbG8=\"".into()],
            })),
        );
        let out = decode_optional_content(&attrs).unwrap().unwrap();
        assert_eq!(out, b"hello");
    }

    #[test]
    fn decode_content_multipart_concat() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "content".to_string(),
            ElementAttribute::Leaf(Box::new(LeafAttribute {
                key: "content".into(),
                tokens: vec!["aGVs".into(), "bG8=".into()],
            })),
        );
        let out = decode_optional_content(&attrs).unwrap().unwrap();
        assert_eq!(out, b"hello");
    }

    #[test]
    fn decode_content_missing() {
        let attrs = HashMap::new();
        assert!(decode_optional_content(&attrs).unwrap().is_none());
    }
}
