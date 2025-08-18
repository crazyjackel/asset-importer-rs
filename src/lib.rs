use asset_importer_rs_core::AiPostProcessSteps;
#[cfg(feature = "gltf2")]
use asset_importer_rs_gltf::{Gltf2Exporter, Gltf2Importer, Output as Gltf2Output};
#[cfg(feature = "gltf")]
use asset_importer_rs_gltf_v1::{GltfExporter, GltfImporter, Output as GltfOutput};
#[cfg(feature = "obj")]
use asset_importer_rs_obj::ObjImporter;
#[cfg(feature = "post-process")]
use asset_importer_rs_post_process::AiPostProcesser;
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

use crate::error::AiExporterError;
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

    #[cfg(feature = "post-process")]
    pub fn from_file_with_post_process(
        file_path: &str,
        flags: BitFlags<AiPostProcessSteps>,
    ) -> Result<AiScene, AiImporterError> {
        let importer = AssetImporter::importer();
        let mut scene = importer.import_file(file_path)?;
        let mut post_process = AiPostProcesser::post_process();
        post_process
            .process(&mut scene, flags)
            .map_err(AiImporterError::PostProcessError)?;
        Ok(scene)
    }

    pub fn export_file(scene: &AiScene, file_path: &str) -> Result<(), AiExporterError> {
        let exporter = AssetImporter::exporter();
        Ok(())
    }

    #[cfg(feature = "post-process")]
    pub fn export_file_with_post_process(
        scene: &mut AiScene,
        file_path: &str,
        flags: BitFlags<AiPostProcessSteps>,
    ) -> Result<(), AiExporterError> {
        let mut post_process = AiPostProcesser::post_process();
        post_process
            .process(scene, flags)
            .map_err(AiExporterError::PostProcessError)?;
        let exporter = AssetImporter::exporter();
        Ok(())
    }
}
