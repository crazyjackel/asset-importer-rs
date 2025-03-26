use asset_importer_rs::{
    core::{import::AiImport, importer::AiImporter},
    formats::{gltf::gltf_importer::GltfImporter, gltf2::default_file_loader},
};

#[test]
fn test_gltf_read_file() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf");
    exe_path.push("Avocado_v1.glb");
    let path = exe_path.as_path();

    let importer = GltfImporter;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");
}
