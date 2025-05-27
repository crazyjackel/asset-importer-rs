use asset_importer_rs_core::{AiImporterExt, default_file_loader};
use asset_importer_rs_gltf_v1::GltfImporter;

#[test]
fn test_gltf_read_file() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf");
    exe_path.push("Avocado_v1.glb");
    let path = exe_path.as_path();

    let importer = GltfImporter;
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "");
}
