use std::{fs::File, io::BufReader};

use asset_importer_rs_core::{AiReadError, Importer};
use asset_importer_rs_gltf::Gltf2Importer;
use asset_importer_rs_gltf_v1::GltfImporter;
use asset_importer_rs_scene::AiScene;

pub struct AssetImporter;

impl AssetImporter {
    pub fn from_file(file_path: &str) -> Result<AiScene, AiReadError> {
        let importer = Importer::new(vec![
            #[cfg(feature = "gltf2")]
            Box::new(Gltf2Importer::new()),
            #[cfg(feature = "gltf")]
            Box::new(GltfImporter::new()),
        ]);
        importer.import_file(file_path)
    }
}
