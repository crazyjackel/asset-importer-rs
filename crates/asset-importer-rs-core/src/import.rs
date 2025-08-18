use std::{
    fs::File,
    io::{self, BufReader, Read, Seek},
    path::Path,
};

use std::error::Error;

use asset_importer_rs_scene::AiScene;

use super::importer_desc::AiImporterDesc;

/// Trait for types that provide information about an importer.
///
/// This trait allows importers to describe their capabilities, supported formats,
/// and other metadata that can be used to select the appropriate importer.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::import::AiImporterInfo;
/// use asset_importer_rs_core::importer_desc::AiImporterDesc;
///
/// struct MyImporter;
///
/// impl AiImporterInfo for MyImporter {
///     fn info(&self) -> AiImporterDesc {
///         // Return importer description
///         todo!()
///     }
/// }
/// ```
///
/// # Methods
///
/// - [`info`]: Returns a description of the importer's capabilities and metadata.
pub trait AiImporterInfo {
    /// Returns information about this importer.
    ///
    /// # Returns
    ///
    /// * An [`AiImporterDesc`] containing metadata about the importer.
    fn info(&self) -> AiImporterDesc;
}

/// Trait for types that can both read and seek.
///
/// This trait combines the [`Read`] and [`Seek`] traits to provide a common interface
/// for data sources that support both reading and seeking operations.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::import::ReadSeek;
/// use std::fs::File;
/// use std::io::{Read, Seek};
///
/// // File implements both Read and Seek, so it automatically implements ReadSeek
/// let file = File::open("example.txt")?;
/// let _: &dyn ReadSeek = &file;
/// ```
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek + ?Sized> ReadSeek for T {}

/// A type alias for a function that loads data from a path
pub type DataLoader<'a> = dyn Fn(&Path) -> io::Result<Box<dyn ReadSeek + 'a>> + 'a;

/// Trait for reading [`AiScene`] data from various sources.
///
/// This trait provides the core interface for importing scene data. It uses dynamic
/// dispatch for the data loader to allow flexible input handling from different sources.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::import::{AiImporter, AiImporterInfo, DataLoader};
/// use asset_importer_rs_scene::AiScene;
/// use std::path::Path;
///
/// struct MyImporter;
///
/// impl AiImporterInfo for MyImporter {
///     fn info(&self) -> asset_importer_rs_core::importer_desc::AiImporterDesc {
///         todo!()
///     }
/// }
///
/// impl AiImporter for MyImporter {
///     type Error = std::io::Error;
///
///     fn can_read_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> bool {
///         // Check if this importer can handle the file
///         true
///     }
///
///     fn read_file_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> Result<AiScene, Self::Error> {
///         // Read and parse the file
///         todo!()
///     }
/// }
/// ```
///
/// # Methods
///
/// - [`can_read_dyn`]: Determines if the importer can handle the specified file.
/// - [`read_file_dyn`]: Reads and parses a file into an [`AiScene`].
pub trait AiImporter: AiImporterInfo {
    /// The error type returned by import operations.
    type Error: Error;

    /// Determines if the provided reader data can be interpreted by this importer.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to check.
    /// * `loader` - Function to load data from the path.
    ///
    /// # Returns
    ///
    /// * `true` if this importer can handle the file, `false` otherwise.
    fn can_read_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> bool;

    /// Reads data from a reader instead of a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    /// * `loader` - Function to load data from the path.
    ///
    /// # Returns
    ///
    /// * `Ok(AiScene)` if the import succeeds, or an error of type [`Self::Error`] if it fails.
    fn read_file_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> Result<AiScene, Self::Error>;
}

/// Extension trait providing convenient methods for scene import.
///
/// This trait provides additional methods that make it easier to import scenes with
/// different types of readers and loaders.
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_core::import::{AiImporterExt, AiImporterInfo};
/// use asset_importer_rs_scene::AiScene;
/// use std::path::Path;
///
/// struct MyImporter;
///
/// impl AiImporterInfo for MyImporter {
///     fn info(&self) -> asset_importer_rs_core::importer_desc::AiImporterDesc {
///         todo!()
///     }
/// }
///
/// impl AiImporterExt for MyImporter {
///     type Error = std::io::Error;
///
///     fn can_read<P: AsRef<Path>, R: std::io::Read + std::io::Seek, F: Fn(&Path) -> std::io::Result<R>>(
///         &self,
///         path: P,
///         loader: F,
///     ) -> bool {
///         // Check if this importer can handle the file
///         true
///     }
///
///     fn read_file<P: AsRef<Path>, R: std::io::Read + std::io::Seek, F: Fn(&Path) -> std::io::Result<R>>(
///         &self,
///         path: P,
///         loader: F,
///     ) -> Result<AiScene, Self::Error> {
///         // Read and parse the file
///         todo!()
///     }
/// }
/// ```
///
/// # Methods
///
/// - [`can_read`]: Checks if the importer can handle a file with a generic reader and loader.
/// - [`read_file`]: Reads a file with a generic reader and loader.
/// - [`can_read_default`]: Checks if the importer can handle a file using the default file loader.
/// - [`read_file_default`]: Reads a file using the default file loader.
pub trait AiImporterExt {
    /// The error type returned by import operations.
    type Error: Error;

    /// Determines if the provided reader data can be interpreted by this importer.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to check.
    /// * `loader` - Function to load data from the path.
    ///
    /// # Returns
    ///
    /// * `true` if this importer can handle the file, `false` otherwise.
    fn can_read<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> bool;

    /// Reads data from a reader instead of a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    /// * `loader` - Function to load data from the path.
    ///
    /// # Returns
    ///
    /// * `Ok(AiScene)` if the import succeeds, or an error of type [`Self::Error`] if it fails.
    fn read_file<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> Result<AiScene, Self::Error>;

    /// Determines if the importer can handle a file using the default file loader.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to check.
    ///
    /// # Returns
    ///
    /// * `true` if this importer can handle the file, `false` otherwise.
    fn can_read_default<P: AsRef<Path>>(&self, path: P) -> bool {
        self.can_read(path, default_file_loader)
    }

    /// Reads a file using the default file loader.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    ///
    /// # Returns
    ///
    /// * `Ok(AiScene)` if the import succeeds, or an error of type [`Self::Error`] if it fails.
    fn read_file_default<P: AsRef<Path>>(&self, path: P) -> Result<AiScene, Self::Error> {
        self.read_file(path, default_file_loader)
    }
}

impl<T: AiImporter + ?Sized> AiImporterExt for T {
    type Error = T::Error;
    /// Determines if the provided reader data can be interpreted by this importer
    fn can_read<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> bool {
        self.can_read_dyn(path.as_ref(), &|p| {
            loader(p).map(|r| Box::new(r) as Box<dyn ReadSeek>)
        })
    }
    /// Reads data from a reader instead of a file path
    fn read_file<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> Result<AiScene, Self::Error> {
        self.read_file_dyn(path.as_ref(), &|p| {
            loader(p).map(|r| Box::new(r) as Box<dyn ReadSeek>)
        })
    }
}

pub fn default_file_loader(path: &Path) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
