use std::collections::HashMap;

use asset_importer_rs_core::{AiExport, AiImport, AiImporter, default_file_loader};
use asset_importer_rs_gltf::{Gltf2Exporter, Output};
use asset_importer_rs_gltf_v1::GltfImporter;

#[test]
fn test_gltf_export_file_binary() {
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

    let exporter = Gltf2Exporter {
        output_type: Output::Binary,
    };
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf");
    exe_path_2.push("Avocado_v2.glb");
    let path = exe_path_2.as_path();
    let _ = exporter.export_file(&scene, path, &HashMap::new());
}

#[test]
fn test_gltf_export_file_binary_v2() {
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

    let exporter = Gltf2Exporter {
        output_type: Output::Binary,
    };
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf");
    exe_path_2.push("Avocado_v2.glb");
    let path = exe_path_2.as_path();
    let _ = exporter.export_file(&scene, path, &HashMap::new());
}
