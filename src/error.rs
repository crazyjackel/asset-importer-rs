use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AiImporterError {
    UnsupportedFileExtension(String),
    #[cfg(feature = "obj")]
    ObjImportError(asset_importer_rs_obj::ObjImportError),
    #[cfg(feature = "gltf")]
    GltfImportError(asset_importer_rs_gltf_v1::GLTFImportError),
    #[cfg(feature = "gltf2")]
    Gltf2ImportError(asset_importer_rs_gltf::Gltf2ImportError),
}

impl Display for AiImporterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiImporterError::UnsupportedFileExtension(extension) => {
                write!(f, "Unsupported file extension: {}", extension)
            }
            #[cfg(feature = "obj")]
            AiImporterError::ObjImportError(error) => write!(f, "ObjImportError: {}", error),
            #[cfg(feature = "gltf")]
            AiImporterError::GltfImportError(error) => write!(f, "GltfImportError: {}", error),
            #[cfg(feature = "gltf2")]
            AiImporterError::Gltf2ImportError(error) => write!(f, "Gltf2ImportError: {}", error),
        }
    }
}

impl Error for AiImporterError {}

#[derive(Debug)]
pub enum AiExporterError {
    #[cfg(feature = "gltf")]
    GltfExportError(asset_importer_rs_gltf_v1::GltfExportError),
    #[cfg(feature = "gltf2")]
    Gltf2ExportError(asset_importer_rs_gltf::Gltf2ExportError),
}

impl Display for AiExporterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "gltf")]
            AiExporterError::GltfExportError(error) => write!(f, "GltfExportError: {}", error),
            #[cfg(feature = "gltf2")]
            AiExporterError::Gltf2ExportError(error) => write!(f, "Gltf2ExportError: {}", error),
        }
    }
}
impl Error for AiExporterError {}

#[cfg(feature = "obj")]
impl From<asset_importer_rs_obj::ObjImportError> for AiImporterError {
    fn from(error: asset_importer_rs_obj::ObjImportError) -> Self {
        AiImporterError::ObjImportError(error)
    }
}

#[cfg(feature = "gltf")]
impl From<asset_importer_rs_gltf_v1::GLTFImportError> for AiImporterError {
    fn from(error: asset_importer_rs_gltf_v1::GLTFImportError) -> Self {
        AiImporterError::GltfImportError(error)
    }
}

#[cfg(feature = "gltf")]
impl From<asset_importer_rs_gltf_v1::GltfExportError> for AiExporterError {
    fn from(error: asset_importer_rs_gltf_v1::GltfExportError) -> Self {
        AiExporterError::GltfExportError(error)
    }
}

#[cfg(feature = "gltf2")]
impl From<asset_importer_rs_gltf::Gltf2ExportError> for AiExporterError {
    fn from(error: asset_importer_rs_gltf::Gltf2ExportError) -> Self {
        AiExporterError::Gltf2ExportError(error)
    }
}

#[cfg(feature = "gltf2")]
impl From<asset_importer_rs_gltf::Gltf2ImportError> for AiImporterError {
    fn from(error: asset_importer_rs_gltf::Gltf2ImportError) -> Self {
        AiImporterError::Gltf2ImportError(error)
    }
}
