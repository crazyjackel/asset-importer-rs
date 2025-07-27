use std::error::Error as StdError;
use std::fs::DirEntry;
use std::{fs, path};

use asset_importer_rs_core::{AiExportExt, AiImporterExt, AiReadError, default_file_loader};
use asset_importer_rs_gltf::{Gltf2Exporter, Gltf2Importer, Output};
use asset_importer_rs_gltf_v1::GltfExporter;
use std::collections::HashMap;

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Assets/Models";

// @todo: Make sure these files have tickets for being removed from skip list
// I would like to test these files, however, there is a particular issue that is hard to fix
const SKIP_FILES: [&str; 33] = [
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
    "glTF-Sample-Assets/Models/SpecularSilkPouf/glTF-Binary/SpecularSilkPouf.glb",
    "glTF-Sample-Assets/Models/TransmissionTest/glTF/TransmissionTest.gltf", //Exhausted Data in the image
    "glTF-Sample-Assets/Models/TransmissionTest/glTF-Binary/TransmissionTest.glb",
    "glTF-Sample-Assets/Models/DragonAttenuation/glTF/DragonAttenuation.gltf",
    "glTF-Sample-Assets/Models/DragonAttenuation/glTF-Binary/DragonAttenuation.glb",
    "glTF-Sample-Assets/Models/MandarinOrange/glTF/MandarinOrange.gltf",
    "glTF-Sample-Assets/Models/MandarinOrange/glTF-Binary/MandarinOrange.glb",
    "glTF-Sample-Assets/Models/CesiumMan/glTF/CesiumMan.gltf",
    "glTF-Sample-Assets/Models/CesiumMan/glTF-Embedded/CesiumMan.gltf",
    "glTF-Sample-Assets/Models/CesiumMan/glTF-Binary/CesiumMan.glb",
    "glTF-Sample-Assets/Models/EnvironmentTest/glTF/EnvironmentTest.gltf",
    "glTF-Sample-Assets/Models/TransmissionOrderTest/glTF/TransmissionOrderTest.gltf",
    "glTF-Sample-Assets/Models/TransmissionOrderTest/glTF-Binary/TransmissionOrderTest.glb",
    "glTF-Sample-Assets/Models/ChairDamaskPurplegold/glTF/ChairDamaskPurplegold.gltf",
    "glTF-Sample-Assets/Models/ChairDamaskPurplegold/glTF-Binary/ChairDamaskPurplegold.glb",
];

//These files should be skipped when running in minimal mode
const SKIP_MINIMAL: [&str; 17] = [
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
    "glTF-Sample-Assets/Models/CommercialRefrigerator/glTF/CommercialRefrigerator.gltf", //Requires KHR_materials_transmission
    "glTF-Sample-Assets/Models/CommercialRefrigerator/glTF-Binary/CommercialRefrigerator.glb",
];

fn run_entry(is_minimal: bool, entry: DirEntry) -> Result<(), Box<dyn StdError>> {
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
                    let _ = importer.read_file(gltf_path, default_file_loader)?;
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
                    let _ = importer.read_file(gle_path, default_file_loader)?;
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
                    let _ = importer.read_file(glb_path, default_file_loader)?;
                }
            }
        }
    };
    Ok(())
}

#[test]
fn external_gltf2_import_sample_assets() {
    #[cfg(feature = "minimal")]
    let is_minimal = true;
    #[cfg(not(feature = "minimal"))]
    let is_minimal = false;
    let mut errors = Vec::new();
    let sample_dir_path = path::Path::new(SAMPLE_MODELS_DIRECTORY_PATH);
    for entry in fs::read_dir(sample_dir_path).unwrap() {
        let entry = entry.unwrap();
        let error = run_entry(is_minimal, entry);
        if let Err(error) = error {
            println!("{}", error);
            errors.push(error);
        }
    }

    for error in errors {
        let is_ai_error = !error.is::<AiReadError>();
        assert!(is_ai_error);
    }
}

// These tests check that the gltf2 importer works as expected.

/// This test is to make sure that basic files can be read.
#[test]
fn test_gltf2_read_file() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");
}

/// This test is to make sure `byteStride` works.
#[test]
fn test_gltf2_read_file_roughness() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding
        .join("tests")
        .join("model")
        .join("gltf2")
        .join("compare_roughness");
    exe_path.push("CompareRoughness.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");
}

/// This test is to make sure that rigged elements load in properly
#[test]
fn test_gltf2_read_file_rigged() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("RiggedFigure.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");
}

/// This test is for different primitive modes load in and a range of indices reading in from the same positions buffer works
#[test]
fn test_gltf2_read_file_primitive() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding
        .join("tests")
        .join("model")
        .join("gltf2")
        .join("primitive_modes");
    exe_path.push("MeshPrimitiveModes.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");
}

/// This test is for sparse accessors working
#[test]
fn test_gltf2_read_file_sparse() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("SimpleSparseAccessor.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");
}

#[test]
fn test_gltf2_read_file_clearcoat() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding
        .join("tests")
        .join("model")
        .join("gltf2")
        .join("clearcoat");
    exe_path.push("ClearCoatTest.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "Scene");
}

#[test]
fn test_gltf2_export_file_binary() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");

    let exporter = Gltf2Exporter {
        output_type: Output::Binary,
    };
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf2");
    exe_path_2.push("Avocado2.glb");
    let path = exe_path_2.as_path();
    exporter
        .export_file_default(&scene, path, &HashMap::new())
        .unwrap();
}

#[test]
fn test_gltf2_export_file_gltf_binary() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");

    let exporter = GltfExporter {
        output_type: asset_importer_rs_gltf_v1::Output::Binary,
    };
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf");
    exe_path_2.push("Avocado_GLTF2.glb");
    let path = exe_path_2.as_path();
    exporter
        .export_file_default(&scene, path, &HashMap::new())
        .unwrap();
}

#[test]
fn test_gltf2_export_file_gltf_standard() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let scene = importer.read_file(path, default_file_loader).unwrap();

    let exporter = GltfExporter {
        output_type: asset_importer_rs_gltf_v1::Output::Standard,
    };
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf");
    exe_path_2.push("Avocado_GLTF2.gltf");
    let path = exe_path_2.as_path();
    exporter
        .export_file_default(&scene, path, &HashMap::new())
        .unwrap();
}
