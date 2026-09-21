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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn import_metadata_from(asset_inner: &str) -> AiMetadata {
        let xml = format!(
            r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    {asset_inner}
  </asset>
</COLLADA>"##
        );
        let document = Document::from_str(&xml).expect("document should parse");
        DaeImporter::new()
            .import_metadata(&document)
            .expect("metadata")
    }

    fn metadata_str<'a>(metadata: &'a AiMetadata, key: &str) -> &'a str {
        match metadata.get(key) {
            Some(AiMetadataEntry::AiStr(s)) => s,
            other => panic!("expected string metadata for {key}, got {other:?}"),
        }
    }

    #[test]
    fn maps_asset_fields_to_ai_metadata_keys() {
        let metadata = import_metadata_from(
            r#"
            <contributor>
              <author>First Author</author>
              <authoring_tool>Tool A</authoring_tool>
              <comments>First comments</comments>
              <copyright>Copyright A</copyright>
              <source_data>file:///first.blend</source_data>
            </contributor>
            <contributor>
              <author>Second Author</author>
              <authoring_tool>Tool B</authoring_tool>
              <comments>Second comments</comments>
              <copyright>Copyright B</copyright>
              <source_data>file:///second.blend</source_data>
            </contributor>
            <created>2018-10-25T16:29:03Z</created>
            <keywords>alpha beta gamma</keywords>
            <modified>2018-10-26T00:00:00</modified>
            <revision>3</revision>
            <subject>Test subject</subject>
            <title>Test title</title>
            <unit meter="2"/>
            <up_axis>Z_UP</up_axis>
            "#,
        );

        assert_eq!(
            metadata_str(&metadata, AI_METADATA_SOURCE_GENERATOR),
            "Tool A"
        );
        assert_eq!(
            metadata_str(&metadata, AI_METADATA_SOURCE_COPYRIGHT),
            "Copyright A"
        );
        assert_eq!(metadata_str(&metadata, AI_COLLADA_AUTHOR), "First Author");
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_COMMENTS),
            "First comments"
        );
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_SOURCE_DATA),
            "file:///first.blend"
        );
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_CREATED),
            "2018-10-25T16:29:03+00:00"
        );
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_MODIFIED),
            "2018-10-26T00:00:00"
        );
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_KEYWORDS),
            "alpha beta gamma"
        );
        assert_eq!(metadata_str(&metadata, AI_COLLADA_REVISION), "3");
        assert_eq!(metadata_str(&metadata, AI_COLLADA_SUBJECT), "Test subject");
        assert_eq!(metadata_str(&metadata, AI_COLLADA_TITLE), "Test title");
        assert!(!metadata.contains_key("Unit"));
        assert!(!metadata.contains_key("UpAxis"));
    }

    #[test]
    fn skips_empty_optional_fields() {
        let metadata = import_metadata_from(
            r#"
            <contributor>
              <author></author>
              <authoring_tool></authoring_tool>
              <comments></comments>
              <copyright></copyright>
            </contributor>
            <created>1970-01-01T00:00:00Z</created>
            <modified>1970-01-01T00:00:00Z</modified>
            <revision></revision>
            <subject></subject>
            <title></title>
            "#,
        );

        assert_eq!(metadata.len(), 2);
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_CREATED),
            "1970-01-01T00:00:00+00:00"
        );
        assert_eq!(
            metadata_str(&metadata, AI_COLLADA_MODIFIED),
            "1970-01-01T00:00:00+00:00"
        );
        assert!(!metadata.contains_key(AI_METADATA_SOURCE_GENERATOR));
        assert!(!metadata.contains_key(AI_METADATA_SOURCE_COPYRIGHT));
        assert!(!metadata.contains_key(AI_COLLADA_AUTHOR));
        assert!(!metadata.contains_key(AI_COLLADA_COMMENTS));
        assert!(!metadata.contains_key(AI_COLLADA_SOURCE_DATA));
        assert!(!metadata.contains_key(AI_COLLADA_KEYWORDS));
        assert!(!metadata.contains_key(AI_COLLADA_REVISION));
        assert!(!metadata.contains_key(AI_COLLADA_SUBJECT));
        assert!(!metadata.contains_key(AI_COLLADA_TITLE));
    }
}
