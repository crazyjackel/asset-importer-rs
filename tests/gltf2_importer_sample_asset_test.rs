use std::error::Error as StdError;
use std::{fs, path};

use asset_importer_rs::core::error::AiReadError;
use asset_importer_rs::core::import::AiImport;
use asset_importer_rs::core::importer::AiImporter;
use asset_importer_rs::formats::gltf2::default_file_loader;
use asset_importer_rs::formats::gltf2::gltf2_importer::Gltf2Importer;

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Assets/Models";

// @todo: Make sure these files have tickets for being removed from skip list
// I would like to test these files, however, there is a particular issue that is hard to fix
const SKIP_FILES: [&str; 19] = [
    "glTF-Sample-Assets/Models/SheenWoodLeatherSofa/glTF/SheenWoodLeatherSofa.gltf", //Sheen Wood Leather Sofa using WebP files which are not fully supported by dependency ATM
    "glTF-Sample-Assets/Models/SheenWoodLeatherSofa/glTF-Binary/SheenWoodLeatherSofa.glb",
    "glTF-Sample-Assets/Models/AnimationPointerUVs/glTF/AnimationPointerUVs.gltf", //Animation Pointers don't work and missing field node is not fixed in 1.4.1
    "glTF-Sample-Assets/Models/AnimationPointerUVs/glTF-Binary/AnimationPointerUVs.glb",
    "glTF-Sample-Assets/Models/AnimatedColorsCube/glTF/AnimatedColorsCube.gltf",
    "glTF-Sample-Assets/Models/AnimatedColorsCube/glTF-Binary/AnimatedColorsCube.glb",
    "glTF-Sample-Assets/Models/PotOfCoalsAnimationPointer/glTF/PotOfCoalsAnimationPointer.gltf",
    "glTF-Sample-Assets/Models/PotOfCoalsAnimationPointer/glTF-Binary/PotOfCoalsAnimationPointer.glb",
    "glTF-Sample-Assets/Models/ClearCoatCarPaint/glTF/ClearCoatCarPaint.gltf", // Clear Coat not available yet
    "glTF-Sample-Assets/Models/ClearCoatCarPaint/glTF-Binary/ClearCoatCarPaint.glb",
    "glTF-Sample-Assets/Models/IridescenceSuzanne/glTF-Binary/IridescenceSuzanne.glb", //Iridescence not supported yet
    "glTF-Sample-Assets/Models/IridescenceSuzanne/glTF/IridescenceSuzanne.gltf",
    "glTF-Sample-Assets/Models/SheenCloth/glTF/SheenCloth.gltf", //Sheen not supported yet
    "glTF-Sample-Assets/Models/SheenChair/glTF/SheenChair.gltf",
    "glTF-Sample-Assets/Models/SheenChair/glTF-Binary/SheenChair.glb",
    "glTF-Sample-Assets/Models/SheenTestGrid/glTF/SheenTestGrid.gltf",
    "glTF-Sample-Assets/Models/SheenTestGrid/glTF-Binary/SheenTestGrid.glb",
    "glTF-Sample-Assets/Models/SpecularSilkPouf/glTF/SpecularSilkPouf.gltf",
    "glTF-Sample-Assets/Models/SpecularSilkPouf/glTF-Binary/SpecularSilkPouf.glb"
];

//These files should be skipped when running in minimal mode
const SKIP_MINIMAL: [&str; 15] = [
    "glTF-Sample-Assets/Models/DirectionalLight/glTF/DirectionalLight.gltf", //Requires Lights to Import
    "glTF-Sample-Assets/Models/DirectionalLight/glTF-Binary/DirectionalLight.glb",
    "glTF-Sample-Assets/Models/PlaysetLightTest/glTF-Binary/PlaysetLightTest.glb",
    "glTF-Sample-Assets/Models/NodePerformanceTest/glTF-Binary/NodePerformanceTest.glb",
    "glTF-Sample-Assets/Models/SpecGlossVsMetalRough/glTF/SpecGlossVsMetalRough.gltf", //Requires KHR_materials_pbrSpecularGlossiness
    "glTF-Sample-Assets/Models/SpecGlossVsMetalRough/glTF-Binary/SpecGlossVsMetalRough.glb",
    "glTF-Sample-Assets/Models/TextureTransformMultiTest/glTF/TextureTransformMultiTest.gltf", //Requires KHR_texture_transform
    "glTF-Sample-Assets/Models/TextureTransformMultiTest/glTF-Binary/TextureTransformMultiTest.glb",
    "glTF-Sample-Assets/Models/TextureTransformTest/glTF/TextureTransformTest.gltf",
    "glTF-Sample-Assets/Models/GlamVelvetSofa/glTF/GlamVelvetSofa.gltf",
    "glTF-Sample-Assets/Models/GlamVelvetSofa/glTF-Binary/GlamVelvetSofa.glb",
    "glTF-Sample-Assets/Models/UnlitTest/glTF/UnlitTest.gltf", //Requires KHR_materials_unlit
    "glTF-Sample-Assets/Models/UnlitTest/glTF-Binary/UnlitTest.glb",
    "glTF-Sample-Assets/Models/DiffuseTransmissionTest/glTF/DiffuseTransmissionTest.gltf",
    "glTF-Sample-Assets/Models/DiffuseTransmissionTest/glTF-Binary/DiffuseTransmissionTest.glb",
];

fn run(is_minimal: bool) -> Result<(), Box<dyn StdError>> {
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
                    let display = format!("{}", gltf_path.display());
                    if SKIP_FILES.contains(&display.as_str())
                        || (is_minimal && SKIP_MINIMAL.contains(&display.as_str()))
                    {
                        println!("Skipping {}", display);
                    } else {
                        println!("Importing {}", display);
                        let importer = Gltf2Importer;
                        let mut ai_importer = AiImporter::default();
                        let _ =
                            importer.read_file(&mut ai_importer, gltf_path, default_file_loader)?;
                    }
                }

                // Import standard glTF with embedded buffer and image data.
                let mut gle_path = entry_path.join("glTF-Embedded").join(file_name);
                gle_path.set_extension("gltf");
                if gle_path.exists() {
                    let display = format!("{}", gle_path.display());
                    if SKIP_FILES.contains(&display.as_str())
                        || (is_minimal && SKIP_MINIMAL.contains(&display.as_str()))
                    {
                        println!("Skipping {}", display);
                    } else {
                        println!("Importing {}", display);
                        let importer = Gltf2Importer;
                        let mut ai_importer = AiImporter::default();
                        let _ =
                            importer.read_file(&mut ai_importer, gle_path, default_file_loader)?;
                    }
                }

                // Import binary glTF.
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    let display = format!("{}", glb_path.display());
                    if SKIP_FILES.contains(&display.as_str())
                        || (is_minimal && SKIP_MINIMAL.contains(&display.as_str()))
                    {
                        println!("Skipping {}", display);
                    } else {
                        println!("Importing {}", display);
                        let importer = Gltf2Importer;
                        let mut ai_importer = AiImporter::default();
                        let _ =
                            importer.read_file(&mut ai_importer, glb_path, default_file_loader)?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
fn external_gltf2_import_sample_assets() {
    #[cfg(feature = "minimal")]
    let is_minimal = true;
    #[cfg(not(feature = "minimal"))]
    let is_minimal = false;
    if let Err(error) = run(is_minimal) {
        let is_ai_error = !error.is::<AiReadError>();
        println!("{}", error);
        assert!(is_ai_error);
    }
}
