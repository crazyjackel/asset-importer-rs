use std::{
    io::{BufRead, BufReader},
    path::Path,
};

use asset_importer_rs_core::{
    AiImporter, AiImporterDesc, AiImporterFlags, AiImporterInfo, AiReadError, DataLoader,
};
use asset_importer_rs_scene::{AiNodeTree, AiScene};
use enumflags2::BitFlags;
use tobj::{LoadError, LoadOptions, load_mtl_buf, load_obj_buf};

use crate::importer::{material::ImportMaterials, mesh::ImportMeshes};

mod material;
mod mesh;

pub struct ObjImporter;

impl ObjImporter {
    pub fn new() -> Self {
        Self
    }
}

impl AiImporterInfo for ObjImporter {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "Wavefront Object Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: "surfaces not supported".to_string(),
            flags: BitFlags::from(AiImporterFlags::Experimental),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["obj".to_string()],
        }
    }
}

impl AiImporter for ObjImporter {
    fn can_read_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> bool {
        match path.extension() {
            None => {
                return false;
            }
            Some(os_str) => match os_str.to_str() {
                Some("obj") => {}
                Some(_) | None => {
                    return false;
                }
            },
        }

        let reader_result = loader(path);
        if reader_result.is_err() {
            return false;
        }
        let reader = reader_result.unwrap();
        let mut buf_reader = BufReader::new(reader);

        //@todo: Assimp uses an approximate method to determine if a file is an obj file.
        // It is debatable whether this is a good idea or not.
        // Determine whether we should do this after further research.

        // read the first five lines making sure they begin with header tokens
        // const HEADER_TOKENS: [&str; 9] = [
        //     "mtllib", "usemtl", "v ", "vt ", "vn ", "o ", "g ", "s ", "f ",
        // ];

        // let mut score: u32 = 0;
        for _ in 0..5 {
            let mut line = String::new();
            let line_result = buf_reader.read_line(&mut line);
            if line_result.is_err() {
                return false;
            }
            // let read_length = line_result.unwrap();
            // if read_length == 0 {
            //     // if this line is empty, we've reached the end of the file
            //     return true;
            // }

            // if HEADER_TOKENS.iter().any(|token| line.starts_with(token)) {
            //     score += 1;
            // }
        }
        true
    }
    fn read_file_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> Result<AiScene, AiReadError> {
        let reader = loader(path).map_err(|x| AiReadError::FileOpenError(Box::new(x)))?;
        let mut buf_reader = BufReader::new(reader);
        let options = LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        };
        let (models, material_result) = load_obj_buf(&mut buf_reader, &options, |mtl_path| {
            let loader_path = path.with_file_name(mtl_path);
            let reader_result = loader(&loader_path).map_err(|_| LoadError::OpenFileFailed)?;
            let mut mtl_buf_reader = BufReader::new(reader_result);
            load_mtl_buf(&mut mtl_buf_reader)
        })
        .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        let materials = material_result.map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or("scene");
        let mut scene = AiScene {
            name: name.to_string(),
            nodes: AiNodeTree::with_root(),
            ..AiScene::default()
        };

        let ImportMaterials(ai_materials, ai_textures) =
            ObjImporter::import_materials(path, materials, loader)?;
        scene.materials = ai_materials;
        scene.textures = ai_textures;

        //Note: material indexes match mesh material indexes
        let ImportMeshes(ai_meshes, ai_nodes) = ObjImporter::import_meshes(models);
        scene.meshes = ai_meshes;
        for node in ai_nodes {
            scene.nodes.insert(node, scene.nodes.root).unwrap();
        }

        Ok(scene)
    }
}
