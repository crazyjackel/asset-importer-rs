mod exporter;
mod importer;

use asset_importer_rs_scene::AiTextureType;
pub use exporter::{Gltf2ExportError, Gltf2Exporter, Output};
pub use importer::{Gltf2ImportError, Gltf2Importer, MeshError};

pub const AI_METADATA_SOURCE_FORMAT: &str = "SourceAsset_Format";
pub const AI_METADATA_SOURCE_FORMAT_VERSION: &str = "SourceAsset_FormatVersion";
pub const AI_METADATA_SOURCE_GENERATOR: &str = "SourceAsset_Generator";
pub const AI_METADATA_SOURCE_COPYRIGHT: &str = "SourceAsset_Copyright";

pub const AI_MATKEY_GLTF_PBRMETALLICROUGHNESS_METALLICROUGHNESS_TEXTURE: AiTextureType =
    AiTextureType::Unknown;
pub const AI_MATKEY_GLTF_ALPHAMODE: &str = "$mat.gltf.alphaMode";
pub const AI_MATKEY_GLTF_ALPHACUTOFF: &str = "$mat.gltf.alphaCutoff";

pub const _AI_MATKEY_GLTF_MAPPINGNAME_BASE: &str = "$tex.mappingname";
pub const _AI_MATKEY_GLTF_MAPPINGID_BASE: &str = "$tex.mappingid";
pub const _AI_MATKEY_GLTF_MAPPINGFILTER_MAG_BASE: &str = "$tex.mappingfiltermag";
pub const _AI_MATKEY_GLTF_MAPPINGFILTER_MIN_BASE: &str = "$tex.mappingfiltermin";
pub const _AI_MATKEY_GLTF_SCALE_BASE: &str = "$tex.scale";
pub const _AI_MATKEY_GLTF_STRENGTH_BASE: &str = "$tex.strength";
