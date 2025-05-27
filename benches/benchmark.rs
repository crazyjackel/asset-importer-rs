use std::{collections::HashMap, time::Duration};

use asset_importer_rs_core::{AiExport, AiImport, AiImporter, default_file_loader};
use asset_importer_rs_gltf::{Gltf2Exporter, Gltf2Importer, Output};
use asset_importer_rs_gltf_v1::GltfImporter;
use criterion::{Criterion, criterion_group, criterion_main};

fn read_file_gltf() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf");
    exe_path.push("Avocado_v1.glb");
    let path = exe_path.as_path();

    let importer: GltfImporter = GltfImporter;
    let mut ai_importer = AiImporter::default();
    let _ = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
}

/// Reads Avocado File as a Benchmark
fn read_file_gltf2() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model").join("gltf2");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let _ = importer
        .read_file(&mut ai_importer, path, default_file_loader)
        .unwrap();
}

/// Reads and Re-Exports Avocado File as a Benchmark
fn export_file_gltf2() {
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
    let mut exe_path_2 = binding.join("test").join("output");
    exe_path_2.push("Avocado2.glb");
    let path = exe_path_2.as_path();
    let _ = exporter.export_file(&scene, path, &HashMap::new());
}

fn criterion_benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("Import Group");
        group.sample_size(10);
        group.measurement_time(Duration::from_millis(1500));
        group.bench_function("import avocado (gltf2)", |b| b.iter(read_file_gltf2));
        group.bench_function("import avocado (gltf)", |b| b.iter(read_file_gltf));
    }
    {
        let mut group_2 = c.benchmark_group("Export Group");
        group_2.sample_size(10);
        group_2.measurement_time(Duration::from_millis(3000));
        group_2.bench_function("export avocado (gltf2)", |b| b.iter(export_file_gltf2));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
