use gltf::Document;

use crate::{core::error::AiReadError, structs::AiMetadata};

use super::gltf2_importer::Gltf2Importer;

pub const AI_METADATA_SOURCE_FORMAT: &str = "SourceAsset_Format";
pub const AI_METADATA_SOURCE_FORMAT_VERSION: &str = "SourceAsset_FormatVersion";
pub const AI_METADATA_SOURCE_GENERATOR: &str = "SourceAsset_Generator";
pub const AI_METADATA_SOURCE_COPYRIGHT: &str = "SourceAsset_Copyright";

impl Gltf2Importer {
    pub(crate) fn import_metadata(document: &Document) -> Result<AiMetadata, AiReadError> {
        let asset = &document.as_json().asset;
        let mut ai_metadata = AiMetadata::new();

        ai_metadata.insert(
            AI_METADATA_SOURCE_FORMAT_VERSION.to_string(),
            crate::structs::AiMetadataEntry::AiStr(asset.version.to_string()),
        );

        if let Some(generator) = &asset.generator {
            ai_metadata.insert(
                AI_METADATA_SOURCE_GENERATOR.to_string(),
                crate::structs::AiMetadataEntry::AiStr(generator.to_string()),
            );
        }

        if let Some(copyright) = &asset.copyright {
            ai_metadata.insert(
                AI_METADATA_SOURCE_COPYRIGHT.to_string(),
                crate::structs::AiMetadataEntry::AiStr(copyright.to_string()),
            );
        }

        Ok(ai_metadata)
    }
}

#[test]
fn test_gltf2_metadata_import() {
    let gltf_data = r#"{
            "asset" : {
                "generator": "glTF Tools for Unity",
                "version" : "2.0"
            }
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let metadata = Gltf2Importer::import_metadata(&document).unwrap();
    assert_eq!(2, metadata.len());
}
