use std::{
    fs::File,
    io::{self, BufReader, Read, Seek},
    path::Path,
};

use asset_importer_rs_scene::AiScene;

use super::{error::AiReadError, importer_desc::AiImporterDesc};

/// A trait for types that provide information about an importer
pub trait AiImporterInfo {
    fn info(&self) -> AiImporterDesc;
}

/// A trait for types that can read and seek
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek + ?Sized> ReadSeek for T {}

/// A type alias for a function that loads data from a path
pub type DataLoader<'a> = dyn Fn(&Path) -> io::Result<Box<dyn ReadSeek + 'a>> + 'a;

/// A trait for reading AiScene from a path
pub trait AiImporter: AiImporterInfo {
    /// Determines if the provided reader data can be interpreted by this importer
    fn can_read_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> bool;
    /// Reads data from a reader instead of a file path
    fn read_file_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> Result<AiScene, AiReadError>;
}

pub trait AiImporterExt {
    /// Determines if the provided reader data can be interpreted by this importer
    fn can_read<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> bool;

    /// Reads data from a reader instead of a file path
    fn read_file<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> Result<AiScene, AiReadError>;

    fn can_read_default<P: AsRef<Path>>(&self, path: P) -> bool {
        self.can_read(path, default_file_loader)
    }

    fn read_file_default<P: AsRef<Path>>(&self, path: P) -> Result<AiScene, AiReadError> {
        self.read_file(path, default_file_loader)
    }
}

impl<T: AiImporter + ?Sized> AiImporterExt for T {
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
    ) -> Result<AiScene, AiReadError> {
        self.read_file_dyn(path.as_ref(), &|p| {
            loader(p).map(|r| Box::new(r) as Box<dyn ReadSeek>)
        })
    }
}

pub fn default_file_loader(path: &Path) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
