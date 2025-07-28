use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

use asset_importer_rs_scene::{AiMatrix4x4, AiReal, AiScene};

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
    type Error: Error;

    fn export_file_dyn(
        &self,
        scene: &AiScene,
        path: &Path,
        properties: &ExportProperties,
        exporter: &DataExporter<'_>,
    ) -> Result<(), Self::Error>;
}

pub trait AiExportExt {
    type Error: Error;

    fn export_file<P: AsRef<Path>, R: Write, F: Fn(&Path) -> io::Result<R>>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
        exporter: F,
    ) -> Result<(), Self::Error>;

    fn export_file_default<P>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
    ) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        self.export_file(scene, path, properties, default_file_exporter)
    }
}

impl<T: AiExport + ?Sized> AiExportExt for T {
    type Error = T::Error;

    fn export_file<P: AsRef<Path>, R: Write, F: Fn(&Path) -> io::Result<R>>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
        exporter: F,
    ) -> Result<(), Self::Error> {
        self.export_file_dyn(scene, path.as_ref(), properties, &|p| {
            exporter(p).map(|w| Box::new(w) as Box<dyn Write>)
        })
    }
}

pub fn default_file_exporter(path: &Path) -> io::Result<BufWriter<File>> {
    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}
