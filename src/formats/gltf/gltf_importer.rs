use enumflags2::BitFlags;

use crate::core::{import::AiImport, importer::AiImporter, importer_desc::{AiImporterDesc, AiImporterFlags}};

#[derive(Debug)]
struct GltfImporter;

impl AiImport for GltfImporter{
    fn can_read(&self, file:&str) -> bool {
        todo!()
    }

    fn read_file(&self, importer: &mut AiImporter, file: &str) -> Result<crate::structs::scene::AiScene, crate::core::error::AiReadError> {
        todo!()
    }

    fn info(&self) -> AiImporterDesc {
        AiImporterDesc{
            name: "glTF Importer".to_string(),
            author: Default::default(),
            maintainer: Default::default(),
            comments: Default::default(),
            flags: BitFlags::from(AiImporterFlags::Experimental),
            min_major: 0,
            min_minor: 0,
            max_major: 0,
            max_minor: 0,
            extensions: vec!["gltf".to_string(),"glb".to_string()],
        }
    }
}

impl GltfImporter{

}