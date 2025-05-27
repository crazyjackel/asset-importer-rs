use std::collections::HashMap;

use asset_importer_rs_core::default_file_loader;
use asset_importer_rs_core::{AiExport, AiImport, AiImporter};
use asset_importer_rs_gltf::{Gltf2Exporter, Gltf2Importer, Output};

#[test]
fn test_gltf2_export_file_binary() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
    assert_eq!(scene.name, "");

    let exporter = Gltf2Exporter {
        output_type: Output::Binary,
    };
    let mut exe_path_2 = binding.join("test").join("output").join("gltf2");
    exe_path_2.push("Avocado2.glb");
    let path = exe_path_2.as_path();
    let _ = exporter.export_file(&scene, path, &HashMap::new());
}
