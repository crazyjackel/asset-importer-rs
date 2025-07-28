use std::{
    io::{self},
    path::Path,
};

use enumflags2::BitFlags;
use gltf_v1::Gltf;

use asset_importer_rs_core::{AiImporter, AiImporterDesc, AiImporterFlags, AiImporterInfo};
use asset_importer_rs_scene::{AiScene, AiSceneFlag};

use camera::ImportCameras;
use light::ImportLights;
use mesh::ImportMeshes;

mod camera;
mod error;
mod light;
mod material;
mod mesh;
mod node;
mod texture;

pub use error::GLTFImportError;

#[derive(Debug, Default)]
pub struct GltfImporter;

impl GltfImporter {
    pub fn new() -> Self {
        Self
    }
}

impl AiImporterInfo for GltfImporter {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "glTF Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: BitFlags::from(AiImporterFlags::Experimental),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["gltf".to_string(), "glb".to_string()],
        }
    }
}

impl AiImporter for GltfImporter {
    type Error = GLTFImportError;

    fn can_read_dyn<'data>(
        &self,
        path: &Path,
        loader: &dyn Fn(&Path) -> io::Result<Box<dyn asset_importer_rs_core::ReadSeek + 'data>>,
    ) -> bool {
        //Match Extension Guard Clause
        match path.extension() {
            None => {
                return false;
            }
            Some(os_str) => match os_str.to_str() {
                Some("gltf") => {}
                Some("glb") => {}
                Some(_) | None => return false,
            },
        };
        //Check if File can be Opened
        let reader_result = loader(path);
        if reader_result.is_err() {
            return false;
        }

        let reader = reader_result.unwrap();
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        gltf.is_ok()
    }

    fn read_file_dyn<'data>(
        &self,
        path: &Path,
        loader: &dyn Fn(&Path) -> io::Result<Box<dyn asset_importer_rs_core::ReadSeek + 'data>>,
    ) -> Result<AiScene, Self::Error> {
        //Collect File Info
        let base = path.parent().unwrap_or_else(|| Path::new("./"));
        let reader =
            loader(path).map_err(|x| GLTFImportError::FileOpenError(x, path.to_path_buf()))?;

        //Load Gltf Info
        let Gltf { document, blob } =
            Gltf::from_reader(reader).map_err(GLTFImportError::FileFormatError)?;

        //@todo: Buffer Data loads all Buffer Data, it would be better to load on an "as-needed case".
        let buffer_data = gltf_v1::import_buffers(&document, Some(base), blob)
            .map_err(GLTFImportError::FileFormatError)?;

        //import textures
        let (embedded_textures, embedded_tex_ids) =
            GltfImporter::import_embedded_textures(&document, Some(base), &buffer_data)?;

        //import materials
        let (embedded_materials, material_index_map) =
            GltfImporter::import_embedded_materials(&document, &embedded_tex_ids)?;

        //import meshes
        let ImportMeshes(meshes, mesh_offsets) =
            GltfImporter::import_meshes(&document, &buffer_data, &material_index_map)?;

        //import cameras
        let ImportCameras(mut cameras, camera_map) = GltfImporter::import_cameras(&document)?;
        //import lights
        let ImportLights(mut lights, light_map) = GltfImporter::import_lights(&document)?;

        let (nodes, name) = GltfImporter::import_nodes(
            &document,
            &mesh_offsets,
            &mut lights,
            &light_map,
            &mut cameras,
            &camera_map,
        )?;

        let mut scene = AiScene {
            textures: embedded_textures,
            materials: embedded_materials,
            meshes,
            cameras,
            lights,
            nodes,
            name,
            ..AiScene::default()
        };

        if !scene.meshes.is_empty() {
            scene.flags |= AiSceneFlag::Incomplete;
        }

        Ok(scene)
    }
}

impl GltfImporter {}
