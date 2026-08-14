use std::path::Path;

use asset_importer_rs_core::{
    AiImporter, AiImporterDesc, AiImporterFlags, AiImporterInfo, DataLoader,
};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

mod error;

pub use error::DaeImportError;

#[derive(Debug, Default)]
pub struct DaeImporter;

impl DaeImporter {
    pub fn new() -> Self {
        Self
    }
}

impl AiImporterInfo for DaeImporter {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "Collada DAE Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: BitFlags::from(AiImporterFlags::Experimental),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["dae".to_string()],
        }
    }
}

impl AiImporter for DaeImporter {
    type Error = DaeImportError;

    fn can_read_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> bool {
        match path.extension() {
            None => {
                return false;
            }
            Some(os_str) => match os_str.to_str() {
                Some("dae") => {}
                Some(_) | None => {
                    return false;
                }
            },
        }

        loader(path).is_ok()
    }

    fn read_file_dyn(
        &self,
        _path: &Path,
        _loader: &DataLoader<'_>,
    ) -> Result<AiScene, Self::Error> {
        Err(DaeImportError::NotImplemented)
    }
}
