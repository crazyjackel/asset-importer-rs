use std::io::{BufReader, Read};
use std::path::Path;

use asset_importer_rs_core::{
    AiImporter, AiImporterDesc, AiImporterFlags, AiImporterInfo, DataLoader,
};
use asset_importer_rs_scene::AiScene;
use dae_parser::Document;
use enumflags2::BitFlags;

use super::DaeImportError;

#[derive(Debug, Default)]
pub struct DaeImporter {
    /// When true, prefer Collada `name` over `id`/`sid` for node names.
    pub use_collada_name: bool,
}

impl DaeImporter {
    pub fn new() -> Self {
        Self::default()
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

        let Ok(mut reader) = loader(path) else {
            return false;
        };

        let mut buf = [0u8; 200];
        let n = match reader.read(&mut buf) {
            Ok(n) => n,
            Err(_) => return false,
        };
        let head = String::from_utf8_lossy(&buf[..n]).to_ascii_lowercase();
        head.contains("<collada")
    }

    fn read_file_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> Result<AiScene, Self::Error> {
        let reader =
            loader(path).map_err(|x| DaeImportError::FileOpenError(x, path.to_path_buf()))?;
        let document = Document::from_reader(BufReader::new(reader))
            .map_err(DaeImportError::FileFormatError)?;

        let visual_scene = document
            .get_visual_scene()
            .ok_or(DaeImportError::MissingVisualScene)?;

        let scene_name = visual_scene
            .name
            .clone()
            .or_else(|| visual_scene.id.clone())
            .unwrap_or_default();

        let (materials, _material_index_map) = self.import_materials(&document)?;
        let nodes = self.import_nodes(&document, visual_scene)?;
        // TODO: import remaining Collada libraries into AiScene
        let animations = Vec::new();
        let cameras = Vec::new();
        let meshes = Vec::new();
        let lights = Vec::new();
        let textures = Vec::new();
        let metadata = Default::default();

        Ok(AiScene {
            name: scene_name,
            animations,
            cameras,
            meshes,
            lights,
            materials,
            textures,
            nodes,
            metadata,
            ..AiScene::default()
        })
    }
}
