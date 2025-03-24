use asset_importer_rs::{
    core::{import::AiImport, importer::AiImporter},
    formats::gltf2::{default_file_loader, gltf2_importer::Gltf2Importer},
};

/// This test is to make sure that basic files can be read.
#[test]
fn test_gltf2_read_file() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");
}

/// This test is to make sure `byteStride` works.
#[test]
fn test_gltf2_read_file_roughness() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding
        .join("tests")
        .join("model")
        .join("compare_roughness");
    exe_path.push("CompareRoughness.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");
}

/// This test is to make sure that rigged elements load in properly
#[test]
fn test_gltf2_read_file_rigged() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model");
    exe_path.push("RiggedFigure.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");
}

/// This test is for different primitive modes load in and a range of indices reading in from the same positions buffer works
#[test]
fn test_gltf2_read_file_primitive() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("primitive_modes");
    exe_path.push("MeshPrimitiveModes.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");
}

/// This test is for sparse accessors working
#[test]
fn test_gltf2_read_file_sparse() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model");
    exe_path.push("SimpleSparseAccessor.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");
}

#[test]
fn test_gltf2_read_file_clearcoat() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("clearcoat");
    exe_path.push("ClearCoatTest.gltf");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "Scene");
}
