mod importer;

pub use asset_importer_rs_core::{
    AI_METADATA_SOURCE_COPYRIGHT, AI_METADATA_SOURCE_GENERATOR,
};
pub use importer::DaeImportError;
pub use importer::DaeImporter;

pub const AI_COLLADA_AUTHOR: &str = "Author";
pub const AI_COLLADA_COMMENTS: &str = "Comments";
pub const AI_COLLADA_SOURCE_DATA: &str = "SourceData";
pub const AI_COLLADA_CREATED: &str = "Created";
pub const AI_COLLADA_MODIFIED: &str = "Modified";
pub const AI_COLLADA_KEYWORDS: &str = "Keywords";
pub const AI_COLLADA_REVISION: &str = "Revision";
pub const AI_COLLADA_SUBJECT: &str = "Subject";
pub const AI_COLLADA_TITLE: &str = "Title";

/// Node metadata when `use_collada_name` is set (Assimp `Collada_id`).
pub const AI_COLLADA_ID: &str = "Collada_id";
/// Node metadata when `use_collada_name` is set (Assimp `Collada_sid`).
pub const AI_COLLADA_SID: &str = "Collada_sid";

