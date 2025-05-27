use std::{collections::HashMap, path::Path};

use asset_importer_rs_scene::{AiMatrix4x4, AiReal, AiScene};

use super::error::AiExportError;

#[derive(Debug, PartialEq, Clone)]
pub enum ExportProperty {
    Int(i32),
    Real(AiReal),
    String(String),
    Matrix(AiMatrix4x4),
}

pub trait AiExport {
    fn export_file<P>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &HashMap<String, ExportProperty>,
    ) -> Result<(), AiExportError>
    where
        P: AsRef<Path>;
}
