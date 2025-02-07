use std::error::Error as StdError;
use std::{fs, path};

use asset_importer_rs::core::error::AiReadError;
use asset_importer_rs::core::import::AiImport;
use asset_importer_rs::core::importer::AiImporter;
use asset_importer_rs::formats::gltf2::gltf2_importer::Gltf2Importer;

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Assets/Models";

fn run() -> Result<(), Box<dyn StdError>> {
    let sample_dir_path = path::Path::new(SAMPLE_MODELS_DIRECTORY_PATH);
    for entry in fs::read_dir(sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                if gltf_path.exists() {
                    println!("Importing {}", gltf_path.display());
                    let importer = Gltf2Importer;
                    let mut ai_importer = AiImporter::default();
                    let _ = importer.read_file(&mut ai_importer, gltf_path)?;
                }

                // Import standard glTF with embedded buffer and image data.
                let mut gle_path = entry_path.join("glTF-Embedded").join(file_name);
                gle_path.set_extension("gltf");
                if gle_path.exists() {
                    println!("Importing {}", gle_path.display());
                    let importer = Gltf2Importer;
                    let mut ai_importer = AiImporter::default();
                    let _ = importer.read_file(&mut ai_importer, gle_path)?;
                }

                // Import binary glTF.
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    println!("Importing {}", glb_path.display());
                    let importer = Gltf2Importer;
                    let mut ai_importer = AiImporter::default();
                    let _ = importer.read_file(&mut ai_importer, glb_path)?;
                }
            }
        }
    }
    Ok(())
}

#[test]
fn import_gltf_sample_assets(){
    if let Err(error) = run() {
        let is_ai_error = !error.is::<AiReadError>();
        println!("{}", error);
        assert!(is_ai_error);
    }
}