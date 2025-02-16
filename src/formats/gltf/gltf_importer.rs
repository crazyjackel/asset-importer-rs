use std::{fs, io::BufReader, path::Path};

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
                Some(_) | None => return false,
            },
        };
        //Check if File can be Opened
        let file_result = fs::File::open(path);
        if file_result.is_err() {
            return false;
        }

        let file = file_result.unwrap();
        let reader = BufReader::new(file);
        let gltf = Gltf::from_reader(reader);

        //If Result is Good, we can Read
        gltf.is_ok()
    }

    fn read_file<P>(
        &self,
        importer: &mut AiImporter,
        path: P,
    ) -> Result<crate::structs::scene::AiScene, crate::core::error::AiReadError>
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
        let buffer_data = gltf_v1::import_buffers(&document, Some(base), blob)
            .map_err(|x| AiReadError::FileFormatError(Box::new(x)))?;

        let mut scene = AiScene {
            ..AiScene::default()
        };

        if !scene.meshes.is_empty() {
            scene.flags |= AiSceneFlag::Incomplete;
        }

        Ok(scene)
    }
}

impl GltfImporter {}
