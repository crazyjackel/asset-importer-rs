use std::collections::HashMap;

use enumflags2::BitFlags;

use asset_importer_rs_core::{AiExportExt, AiImporterExt, AiPostProcessSteps, default_file_loader};
use asset_importer_rs_gltf::{Gltf2Exporter, Output};
use asset_importer_rs_gltf_v1::{GltfExporter, Output as GltfOutput};
use asset_importer_rs_obj::ObjImporter;
#[cfg(feature = "post-process")]
use asset_importer_rs_post_process::AiPostProcesser;

#[test]
fn import_obj_file_export_gltf() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let exe_path = binding
        .join("crates")
        .join("asset-importer-rs-obj")
        .join("assets")
        .join("spider.obj");
    let path = exe_path.as_path();

    let importer = ObjImporter::new();
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "spider");

    let exporter = GltfExporter::new(GltfOutput::Binary);
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf");
    exe_path_2.push("spider.glb");
    let path = exe_path_2.as_path();
    exporter
        .export_file_default(&scene, path, &HashMap::new())
        .unwrap();
}

#[test]
#[cfg(feature = "post-process")]
fn import_obj_file_export_gltf2_with_post_process() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let exe_path = binding
        .join("crates")
        .join("asset-importer-rs-obj")
        .join("assets")
        .join("spider.obj");
    let path = exe_path.as_path();

    let importer = ObjImporter::new();
    let mut scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "spider");

    let mut post_process = AiPostProcesser::post_process();
    post_process
        .process(
            &mut scene,
            BitFlags::from_flag(AiPostProcessSteps::GenSmoothNormals),
        )
        .unwrap();

    let exporter = Gltf2Exporter::new(Output::Binary);
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf2");
    exe_path_2.push("spider_fixed.glb");
    let path = exe_path_2.as_path();
    exporter
        .export_file_default(&scene, path, &HashMap::new())
        .unwrap();
}

#[test]
fn import_obj_file_export_gltf2() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let exe_path = binding
        .join("crates")
        .join("asset-importer-rs-obj")
        .join("assets")
        .join("spider.obj");
    let path = exe_path.as_path();

    let importer = ObjImporter::new();
    let scene = importer.read_file(path, default_file_loader).unwrap();
    assert_eq!(scene.name, "spider");

    let exporter = Gltf2Exporter::new(Output::Binary);
    let mut exe_path_2 = binding.join("tests").join("output").join("gltf2");
    exe_path_2.push("spider.glb");
    let path = exe_path_2.as_path();
    exporter
        .export_file_default(&scene, path, &HashMap::new())
        .unwrap();
}
