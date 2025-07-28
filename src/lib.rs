use asset_importer_rs_core::AiExportError;
#[cfg(feature = "gltf2")]
use asset_importer_rs_gltf::{Gltf2Exporter, Gltf2Importer, Output as Gltf2Output};
#[cfg(feature = "gltf")]
use asset_importer_rs_gltf_v1::{GltfExporter, GltfImporter, Output as GltfOutput};
#[cfg(feature = "obj")]
use asset_importer_rs_obj::ObjImporter;
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

pub use crate::error::AiImporterError;
use crate::exporter::{ExportFormatEntry, Exporter};
pub use crate::importer::Importer;
pub use crate::wrapper::{AiExportWrapper, AiImportWrapper};

mod error;
mod exporter;
mod importer;
mod wrapper;

pub struct AssetImporter;

impl AssetImporter {
    pub fn importer() -> Importer {
        Importer::new(vec![
            #[cfg(feature = "obj")]
            Box::new(AiImportWrapper::new(ObjImporter::new())),
            #[cfg(feature = "gltf2")]
            Box::new(AiImportWrapper::new(Gltf2Importer::new())),
            #[cfg(feature = "gltf")]
            Box::new(AiImportWrapper::new(GltfImporter::new())),
        ])
    }

    pub fn exporter() -> Exporter {
        Exporter::new(vec![
            #[cfg(feature = "gltf2")]
            ExportFormatEntry::new(
                Box::new(AiExportWrapper::new(Gltf2Exporter::new(
                    Gltf2Output::Standard,
                ))),
                "ggltf2".to_string(),
                "GL Transmission Format v. 2".to_string(),
                "gltf".to_string(),
                BitFlags::empty(),
            ),
            #[cfg(feature = "gltf2")]
            ExportFormatEntry::new(
                Box::new(AiExportWrapper::new(Gltf2Exporter::new(
                    Gltf2Output::Binary,
                ))),
                "glb2".to_string(),
                "GL Transmission Format v. 2 (binary)".to_string(),
                "glb".to_string(),
                BitFlags::empty(),
            ),
            #[cfg(feature = "gltf")]
            ExportFormatEntry::new(
                Box::new(AiExportWrapper::new(GltfExporter::new(
                    GltfOutput::Standard,
                ))),
                "gltf".to_string(),
                "GL Transmission Format".to_string(),
                "gltf".to_string(),
                BitFlags::empty(),
            ),
            #[cfg(feature = "gltf")]
            ExportFormatEntry::new(
                Box::new(AiExportWrapper::new(GltfExporter::new(GltfOutput::Binary))),
                "glb".to_string(),
                "GL Transmission Format (binary)".to_string(),
                "glb".to_string(),
                BitFlags::empty(),
            ),
        ])
    }

    pub fn from_file(file_path: &str) -> Result<AiScene, AiImporterError> {
        let importer = AssetImporter::importer();
        importer.import_file(file_path)
    }

    pub fn export_file(scene: &AiScene, file_path: &str) -> Result<(), AiExportError> {
        let exporter = AssetImporter::exporter();
        Ok(())
    }
}
