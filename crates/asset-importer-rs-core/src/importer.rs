use std::{
    collections::HashMap,
    io::{self, BufReader, Cursor},
    path::Path,
    sync::Arc,
};

use asset_importer_rs_scene::AiScene;

use crate::{AiImporter, AiImporterExt, AiReadError, ReadSeek, default_file_loader};

#[derive(Default)]
pub struct Importer {
    importers: Vec<Box<dyn AiImporter>>,
}

impl Importer {
    pub fn new(importers: Vec<Box<dyn AiImporter>>) -> Self {
        Self { importers }
    }

    pub fn import_file(&self, file_path: &str) -> Result<AiScene, AiReadError> {
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

        Err(AiReadError::UnsupportedFileExtension(extension.to_string()))
    }

    pub fn import_from_memory(
        &self,
        file_name: &str,
        data: &HashMap<String, Vec<u8>>,
    ) -> Result<AiScene, AiReadError> {
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

        Err(AiReadError::UnsupportedFileExtension(extension.to_string()))
    }
}
