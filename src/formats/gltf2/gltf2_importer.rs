use std::{fs, io::BufReader, path::Path};

use gltf::Gltf;

use crate::{
    core::{
        error::AiReadError,
        import::AiImport,
        importer::AiImporter,
        importer_desc::{AiImporterDesc, AiImporterFlags},
    },
    structs::scene::{AiScene, AiSceneFlag},
};

#[derive(Debug)]
pub struct Gltf2Importer;

impl AiImport for Gltf2Importer {
    fn info(&self) -> AiImporterDesc {
        AiImporterDesc {
            name: "glTF2 Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: (AiImporterFlags::SupportBinaryFlavor
                    | AiImporterFlags::LimitedSupport
                    | AiImporterFlags::SupportTextFlavor | AiImporterFlags::Experimental),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["gltf".to_string(), "glb".to_string(), "vrm".to_string()],
        }
    }

    fn can_read<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        //Match Extension Guard Clause
        match path.as_ref().extension() {
            None => {
                return false;
            }
            Some(os_str) => match os_str.to_str() {
                Some("gltf") => {}
                Some("glb") => {}
                Some("vrm") => {}
                Some(_) | None => return false,
            },
        };
        //Check if File can be Opened
        let file_result = fs::File::open(path);
        if file_result.is_err() {
            return false;
        }

        //Attempt to Read JSON
        let file = file_result.unwrap();
        let reader = BufReader::new(file);
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        gltf.is_ok()
    }

    fn read_file<P>(&self, _importer: &mut AiImporter, path: P) -> Result<AiScene, AiReadError>
    where
        P: AsRef<std::path::Path>,
    {
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
        let buffer_data = gltf::import_buffers(&document, Some(base), blob)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        //import textures
        let (embedded_textures, embedded_tex_ids) =
            Gltf2Importer::import_embedded_textures(&document, Some(base), &buffer_data)?;
        //import materials
        let embedded_materials =
            Gltf2Importer::import_embedded_materials(&document, &embedded_tex_ids)?;
        //import meshes
        let (mut meshes, mesh_offsets, remapping_tables) =
            Gltf2Importer::import_meshes(&document, &buffer_data, embedded_materials.len() - 1)?;

        //import cameras
        let mut cameras = Gltf2Importer::import_cameras(&document)?;
        //import lights
        let mut lights = Gltf2Importer::import_lights(&document)?;

        //import nodes
        let (nodes, scene_name) = Gltf2Importer::import_nodes(
            &document,
            &buffer_data,
            &mut meshes,
            &mesh_offsets,
            &remapping_tables,
            &mut lights,
            &mut cameras,
        )?;
        
        //import animations
        let animations = Gltf2Importer::import_animations(&document, &buffer_data)?;

        //import metadata
        let metadata = Gltf2Importer::import_metadata(&document)?;

        let mut scene = AiScene{
            name: scene_name,
            animations,
            cameras,
            meshes,
            lights,
            materials: embedded_materials,
            textures: embedded_textures,
            nodes,
            metadata,
            ..AiScene::default()
        };

        if !scene.meshes.is_empty(){
            scene.flags |= AiSceneFlag::Incomplete;
        }

        Ok(scene)
    }
}


#[test]
fn test_read_file(){
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer.read_file(&mut ai_importer, path).unwrap();
    assert_eq!(scene.name, "");
}