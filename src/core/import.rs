use crate::structs::scene::AiScene;

use super::{error::AiReadError, importer::AiImporter, importer_desc::AiImporterDesc};

pub trait AiImport{
    fn can_read(&self, file:&str) -> bool;
    fn read_file(&self, importer: &mut AiImporter, file: &str) -> Result<AiScene, AiReadError>;
    fn info(&self) -> AiImporterDesc;
}

impl dyn AiImport{
    pub fn get_extensions(&self) -> Vec<String>{
        self.info().extensions
    }
}