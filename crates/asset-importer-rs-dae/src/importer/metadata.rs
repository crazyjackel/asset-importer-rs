use asset_importer_rs_scene::{AiMetadata, AiMetadataEntry};
use dae_parser::Document;

use crate::{
    AI_COLLADA_AUTHOR, AI_COLLADA_COMMENTS, AI_COLLADA_CREATED, AI_COLLADA_KEYWORDS,
    AI_COLLADA_MODIFIED, AI_COLLADA_REVISION, AI_COLLADA_SOURCE_DATA, AI_COLLADA_SUBJECT,
    AI_COLLADA_TITLE, AI_METADATA_SOURCE_COPYRIGHT, AI_METADATA_SOURCE_GENERATOR, DaeImportError,
};

use super::DaeImporter;

impl DaeImporter {
    pub(crate) fn import_metadata(
        &self,
        document: &Document,
    ) -> Result<AiMetadata, DaeImportError> {

        let asset = &document.asset;
        let mut metadata = AiMetadata::new();

        for contributor in &asset.contributor {
            insert_first(
                &mut metadata,
                AI_METADATA_SOURCE_GENERATOR,
                contributor.authoring_tool.clone(),
            );
            insert_first(
                &mut metadata,
                AI_METADATA_SOURCE_COPYRIGHT,
                contributor.copyright.clone(),
            );
            insert_first(&mut metadata, AI_COLLADA_AUTHOR, contributor.author.clone());
            insert_first(
                &mut metadata,
                AI_COLLADA_COMMENTS,
                contributor.comments.clone(),
            );
            insert_first(
                &mut metadata,
                AI_COLLADA_SOURCE_DATA,
                contributor.source_data.as_ref().map(ToString::to_string),
            );
        }

        insert_first(
            &mut metadata,
            AI_COLLADA_CREATED,
            Some(asset.created.to_string()),
        );
        insert_first(
            &mut metadata,
            AI_COLLADA_MODIFIED,
            Some(asset.modified.to_string()),
        );
        insert_first(
            &mut metadata,
            AI_COLLADA_KEYWORDS,
            (!asset.keywords.is_empty()).then(|| asset.keywords.join(" ")),
        );
        insert_first(&mut metadata, AI_COLLADA_REVISION, asset.revision.clone());
        insert_first(&mut metadata, AI_COLLADA_SUBJECT, asset.subject.clone());
        insert_first(&mut metadata, AI_COLLADA_TITLE, asset.title.clone());

        Ok(metadata)
    }
}

fn insert_first(metadata: &mut AiMetadata, key: &str, value: Option<String>) {
    if let Some(value) = value.filter(|s| !s.is_empty()) {
        metadata
            .entry(key.to_string())
            .or_insert(AiMetadataEntry::AiStr(value));
    }
}