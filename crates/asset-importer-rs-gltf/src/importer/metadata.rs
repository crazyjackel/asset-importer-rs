use gltf::Document;

use asset_importer_rs_scene::{AiMetadata, AiMetadataEntry};

use crate::importer::error::Gltf2ImportError;

use super::importer::Gltf2Importer;

use crate::{
    AI_METADATA_SOURCE_COPYRIGHT, AI_METADATA_SOURCE_FORMAT_VERSION, AI_METADATA_SOURCE_GENERATOR,
};

impl Gltf2Importer {
    pub(crate) fn import_metadata(document: &Document) -> Result<AiMetadata, Gltf2ImportError> {
        let asset = &document.as_json().asset;
        let mut ai_metadata = AiMetadata::new();

        ai_metadata.insert(
            AI_METADATA_SOURCE_FORMAT_VERSION.to_string(),
            AiMetadataEntry::AiStr(asset.version.to_string()),
        );

        if let Some(generator) = &asset.generator {
            ai_metadata.insert(
                AI_METADATA_SOURCE_GENERATOR.to_string(),
                AiMetadataEntry::AiStr(generator.to_string()),
            );
        }

        if let Some(copyright) = &asset.copyright {
            ai_metadata.insert(
                AI_METADATA_SOURCE_COPYRIGHT.to_string(),
                AiMetadataEntry::AiStr(copyright.to_string()),
            );
        }

        Ok(ai_metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
