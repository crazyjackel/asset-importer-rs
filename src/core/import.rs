use std::{
    io::{self, Read, Seek},
    path::Path,
};

use crate::structs::scene::AiScene;

use super::{error::AiReadError, importer::AiImporter, importer_desc::AiImporterDesc};

pub trait AiImport {
    /// Determines if the provided reader data can be interpreted by this importer
    fn can_read<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> bool;

    /// Reads data from a reader instead of a file path
    fn read_file<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        importer: &mut AiImporter,
        path: P,
        loader: F,
    ) -> Result<AiScene, AiReadError>;

    /// Provides a description of the importer
    fn info(&self) -> AiImporterDesc;
}
