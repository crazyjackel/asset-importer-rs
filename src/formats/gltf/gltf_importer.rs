use std::{
    fs,
    io::{self, BufReader, Read, Seek},
    path::Path,
};

use enumflags2::BitFlags;
use gltf_v1::Gltf;

use crate::{
    core::{
        error::AiReadError,
        import::AiImport,
        importer::AiImporter,
        importer_desc::{AiImporterDesc, AiImporterFlags},
    },
    structs::scene::{AiScene, AiSceneFlag},
};

use super::{
    gltf_importer_camera::ImportCameras, gltf_importer_light::ImportLights,
    gltf_importer_mesh::ImportMeshes,
};

#[derive(Debug)]
pub struct GltfImporter;

impl AiImport for GltfImporter {
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

    fn can_read<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        path: P,
        loader: F,
    ) -> bool {
        //Match Extension Guard Clause
        match path.as_ref().extension() {
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
        let reader_result = loader(path.as_ref());
        if reader_result.is_err() {
            return false;
        }

        let reader = reader_result.unwrap();
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        gltf.is_ok()
    }

    fn read_file<P: AsRef<Path>, R: Read + Seek, F: Fn(&Path) -> io::Result<R>>(
        &self,
        importer: &mut AiImporter,
        path: P,
        loader: F,
    ) -> Result<AiScene, AiReadError> {
        //Collect File Info
        let path_ref = path.as_ref();
        let base = path_ref.parent().unwrap_or_else(|| Path::new("./"));
        let file_result =
            fs::File::open(path_ref).map_err(|x| AiReadError::FileOpenError(Box::new(x)))?;
        let reader = BufReader::new(file_result);

        //Load Gltf Info
        let Gltf { document, blob } =
            Gltf::from_reader(reader).map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //@todo: Buffer Data loads all Buffer Data, it would be better to load on an "as-needed case".
        let buffer_data = gltf_v1::import_buffers(&document, Some(base), blob)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

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
            &buffer_data,
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
