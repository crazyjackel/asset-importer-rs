use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use asset_importer_rs_scene::{AiMatrix4x4, AiReal, AiScene};

use super::error::AiExportError;

#[derive(Debug, PartialEq, Clone)]
pub enum ExportProperty {
    Int(i32),
    Real(AiReal),
    String(String),
    Matrix(AiMatrix4x4),
}

pub type ExportProperties = HashMap<String, ExportProperty>;

/// A type alias for a function that creates a writer for a path
pub type DataExporter<'a> = dyn Fn(&Path) -> io::Result<Box<dyn Write + 'a>> + 'a;

pub trait AiExport {
    fn export_file_dyn(
        &self,
        scene: &AiScene,
        path: &Path,
        properties: &ExportProperties,
        exporter: &DataExporter<'_>,
    ) -> Result<(), AiExportError>;
}

pub trait AiExportExt {
    fn export_file<P: AsRef<Path>, R: Write, F: Fn(&Path) -> io::Result<R>>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
        exporter: F,
    ) -> Result<(), AiExportError>;

    fn export_file_default<P>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
    ) -> Result<(), AiExportError>
    where
        P: AsRef<Path>,
    {
        self.export_file(scene, path, properties, default_file_exporter)
    }
}

impl<T: AiExport + ?Sized> AiExportExt for T {
    fn export_file<P: AsRef<Path>, R: Write, F: Fn(&Path) -> io::Result<R>>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
        exporter: F,
    ) -> Result<(), AiExportError> {
        self.export_file_dyn(scene, path.as_ref(), properties, &|p| {
            exporter(p).map(|w| Box::new(w) as Box<dyn Write>)
        })
    }
}

pub fn default_file_exporter(path: &Path) -> io::Result<BufWriter<File>> {
    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}
