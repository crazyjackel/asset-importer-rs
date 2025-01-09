use std::path::Path;

use crate::structs::scene::AiScene;

use super::{error::AiReadError, importer::AiImporter, importer_desc::AiImporterDesc};

pub trait AiImport{
    fn can_read<P>(&self, path: P) -> bool where P: AsRef<Path>;
    fn read_file<P>(&self, importer: &mut AiImporter, path: P) -> Result<AiScene, AiReadError> where P: AsRef<Path>;
    fn info(&self) -> AiImporterDesc;
}