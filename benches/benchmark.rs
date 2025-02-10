use std::{collections::HashMap, time::Duration};

use asset_importer_rs::{
    core::{export::AiExport, import::AiImport, importer::AiImporter},
    formats::gltf2::{
        gltf2_exporter::{Gltf2Exporter, Output},
        gltf2_importer::Gltf2Importer,
    },
};
use criterion::{criterion_group, criterion_main, Criterion};

/// Reads Avocado File as a Benchmark
fn read_file() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let _ = importer.read_file(&mut ai_importer, path).unwrap();
}

/// Reads and Re-Exports Avocado File as a Benchmark
fn export_file() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let mut exe_path = binding.join("tests").join("model");
    exe_path.push("Avocado.glb");
    let path = exe_path.as_path();

    let importer = Gltf2Importer;
    let mut ai_importer = AiImporter::default();
    let scene = importer.read_file(&mut ai_importer, path).unwrap();
    assert_eq!(scene.name, "");

    let exporter = Gltf2Exporter {
        output_type: Output::Binary,
    };
    let mut exe_path_2 = binding.join("test_output").join("model");
    exe_path_2.push("Avocado2.glb");
    let path = exe_path_2.as_path();
    let _ = exporter.export_file(&scene, path, &HashMap::new());
}

fn criterion_benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("Import Group");
        group.sample_size(10);
        group.measurement_time(Duration::from_millis(1500));
        group.bench_function("import avocado", |b| b.iter(read_file));
    }
    {
        let mut group_2 = c.benchmark_group("Export Group");
        group_2.sample_size(10);
        group_2.measurement_time(Duration::from_millis(3000));
        group_2.bench_function("export avocado", |b| b.iter(export_file));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
