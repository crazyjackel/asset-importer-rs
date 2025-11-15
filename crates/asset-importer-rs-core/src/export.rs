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

/// Trait for implementing scene export functionality.
///
/// This trait provides the core interface for exporting [`AiScene`] data to various file formats.
/// It uses dynamic dispatch for the data exporter to allow flexible output handling.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::{AiExport, ExportProperties};
/// use asset_importer_rs_scene::AiScene;
/// use std::path::Path;
///
/// struct MyExporter;
///
/// impl AiExport for MyExporter {
///     type Error = std::io::Error;
///
///     fn export_file_dyn(
///         &self,
///         _scene: &AiScene,
///         _path: &Path,
///         _properties: &ExportProperties,
///         _exporter: &asset_importer_rs_core::DataExporter<'_>,
///     ) -> Result<(), Self::Error> {
///         // Export the scene to the specified path
///         Ok(())
///     }
/// }
/// ```
///
/// # Methods
///
/// - [`export_file_dyn`]: Exports a scene to a file using dynamic dispatch for the data exporter.
pub trait AiExport {
    /// The error type returned by export operations.
    type Error: Error;

    /// Exports a scene to a file using dynamic dispatch for the data exporter.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to export.
    /// * `path` - The target file path.
    /// * `properties` - Export configuration properties.
    /// * `exporter` - Function to create a writer for the target path.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the export succeeds, or an error of type [`Self::Error`] if it fails.
    fn export_file_dyn(
        &self,
        scene: &AiScene,
        path: &Path,
        properties: &ExportProperties,
        exporter: &DataExporter<'_>,
    ) -> Result<(), Self::Error>;
}

/// Extension trait providing convenient methods for scene export.
///
/// This trait provides additional methods that make it easier to export scenes with
/// different types of writers and exporters.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::{AiExport, AiExportExt, ExportProperties};
/// use asset_importer_rs_scene::AiScene;
///
/// struct MyExporter;
///
/// impl AiExport for MyExporter {
///     type Error = std::io::Error;
///
///     fn export_file_dyn(
///         &self,
///         _scene: &AiScene,
///         _path: &std::path::Path,
///         _properties: &ExportProperties,
///         _exporter: &asset_importer_rs_core::DataExporter<'_>,
///     ) -> Result<(), Self::Error> {
///         Ok(())
///     }
/// }
///
/// // AiExportExt is automatically implemented for types that implement AiExport
/// let exporter = MyExporter;
/// let scene = AiScene::default();
/// let mut properties = ExportProperties::new();
/// // exporter.export_file_default(&scene, "output.txt", &properties)?;
/// ```
///
/// # Methods
///
/// - [`export_file`]: Exports a scene with a generic writer and exporter function.
/// - [`export_file_default`]: Exports a scene using the default file exporter.
pub trait AiExportExt {
    /// The error type returned by export operations.
    type Error: Error;

    /// Exports a scene with a generic writer and exporter function.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to export.
    /// * `path` - The target file path.
    /// * `properties` - Export configuration properties.
    /// * `exporter` - Function to create a writer for the target path.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the export succeeds, or an error of type [`Self::Error`] if it fails.
    fn export_file<P: AsRef<Path>, R: Write, F: Fn(&Path) -> io::Result<R>>(
        &self,
        scene: &AiScene,
        path: P,
        properties: &ExportProperties,
        exporter: F,
    ) -> Result<(), Self::Error>;

    /// Exports a scene using the default file exporter.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to export.
    /// * `path` - The target file path.
    /// * `properties` - Export configuration properties.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the export succeeds, or an error of type [`Self::Error`] if it fails.
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
