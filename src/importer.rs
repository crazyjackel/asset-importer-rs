use std::{
    collections::HashMap,
    io::{self, BufReader, Cursor},
    path::Path,
};

use asset_importer_rs_core::{AiImporter, AiImporterExt, ReadSeek};
use asset_importer_rs_scene::AiScene;

use crate::error::AiImporterError;

type AiImporterDyn = dyn AiImporter<Error = AiImporterError>;

#[derive(Default)]
pub struct Importer {
    importers: Vec<Box<AiImporterDyn>>,
}

impl Importer {
    pub fn new(importers: Vec<Box<AiImporterDyn>>) -> Self {
        Self { importers }
    }

    pub fn import_file(&self, file_path: &str) -> Result<AiScene, AiImporterError> {
        let path = Path::new(file_path);
        let extension = path.extension().unwrap_or_default();
        let extension = extension.to_str().unwrap_or_default();

        for importer in &self.importers {
            if !importer.info().extensions.iter().any(|e| e == extension) {
                continue;
            }

            if importer.can_read_default(path) {
                return importer.read_file_default(path);
            }
        }

        Err(AiImporterError::UnsupportedFileExtension(
            extension.to_string(),
        ))
    }

    pub fn import_from_memory(
        &self,
        file_name: &str,
        data: &HashMap<String, Vec<u8>>,
    ) -> Result<AiScene, AiImporterError> {
        let path = Path::new(file_name);
        let extension = path.extension().unwrap_or_default();
        let extension = extension.to_str().unwrap_or_default();

        let memory_loader = move |path: &Path| -> io::Result<Box<dyn ReadSeek>> {
            if let Some(path_str) = path.to_str() {
                if let Some(data_vec) = data.get(path_str) {
                    let cursor = Cursor::new(data_vec.as_slice());
                    let reader = BufReader::new(cursor);
                    return Ok(Box::new(reader));
                }
            }

            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        };

        for importer in &self.importers {
            if !importer.info().extensions.iter().any(|e| e == extension) {
                continue;
            }

            if importer.can_read_dyn(path, &memory_loader) {
                return importer.read_file_dyn(path, &memory_loader);
            }
        }

        Err(AiImporterError::UnsupportedFileExtension(
            extension.to_string(),
        ))
    }
}
