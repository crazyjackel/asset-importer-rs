use enumflags2::BitFlags;

use asset_importer_rs_core::{AiExport, AiPostProcessSteps};

use crate::error::AiExporterError;

pub struct ExportFormatEntry {
    /// The exporter for this format.
    pub exporter: Box<dyn AiExport<Error = AiExporterError>>,
    /// The name of the format.
    pub name: String,
    /// The description of the format.
    pub description: String,
    /// The extension of the file.
    pub extension: String,
    /// Post processing steps that are enforced for this format.
    pub enforced_post_process_steps: BitFlags<AiPostProcessSteps>,
}

impl ExportFormatEntry {
    pub fn new(
        exporter: Box<dyn AiExport<Error = AiExporterError>>,
        name: String,
        description: String,
        extension: String,
        enforced_post_process_steps: BitFlags<AiPostProcessSteps>,
    ) -> Self {
        Self {
            exporter,
            name,
            description,
            extension,
            enforced_post_process_steps,
        }
    }
}

#[derive(Default)]
pub struct Exporter {
    exporters: Vec<ExportFormatEntry>,
}

impl Exporter {
    pub fn new(exporters: Vec<ExportFormatEntry>) -> Self {
        Self { exporters }
    }
}
