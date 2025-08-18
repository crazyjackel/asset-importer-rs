/// Metadata key for the source asset format.
///
/// This constant is used to store information about the original format
/// of the asset before it was imported into the scene.
pub const AI_METADATA_SOURCE_FORMAT: &str = "SourceAsset_Format";

/// Metadata key for the source asset format version.
///
/// This constant is used to store the version information of the original
/// asset format, which can be useful for compatibility checking.
pub const AI_METADATA_SOURCE_FORMAT_VERSION: &str = "SourceAsset_FormatVersion";

/// Metadata key for the source asset generator.
///
/// This constant is used to store information about the software or tool
/// that originally created the asset, such as "Blender 3.0" or "Maya 2023".
pub const AI_METADATA_SOURCE_GENERATOR: &str = "SourceAsset_Generator";

/// Metadata key for the source asset copyright information.
///
/// This constant is used to store copyright and licensing information
/// about the original asset, which should be preserved during import/export.
pub const AI_METADATA_SOURCE_COPYRIGHT: &str = "SourceAsset_Copyright";
