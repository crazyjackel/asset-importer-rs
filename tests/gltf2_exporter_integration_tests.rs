use std::collections::HashMap;

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_core::{AiExportExt, default_file_loader};
use asset_importer_rs_gltf::{Gltf2Exporter, Gltf2Importer, Output};
use asset_importer_rs_gltf_v1::GltfExporter;

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
