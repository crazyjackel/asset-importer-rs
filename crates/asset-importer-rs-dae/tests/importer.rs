use std::{
    io::{self, Cursor},
    path::Path,
};

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_dae::DaeImporter;

fn dummy_loader(_path: &Path) -> io::Result<Cursor<Vec<u8>>> {
    Ok(Cursor::new(Vec::new()))
}

fn collada_loader(_path: &Path) -> io::Result<Cursor<Vec<u8>>> {
    Ok(Cursor::new(
        b"<?xml version=\"1.0\"?><COLLADA xmlns=\"http://www.collada.org/2005/11/COLLADASchema\" version=\"1.4.1\">"
            .to_vec(),
    ))
}

#[test]
fn test_dae_can_read_extension() {
    let importer = DaeImporter::new();
    assert!(importer.can_read("model.dae", collada_loader));
    assert!(!importer.can_read("model.dae", dummy_loader));
    assert!(!importer.can_read("model.obj", collada_loader));
    assert!(!importer.can_read("model", collada_loader));
}

fn load_cube_scene() -> asset_importer_rs_scene::AiScene {
    let importer = DaeImporter::new();
    let path = Path::new("tests/cube.dae");
    assert!(path.exists(), "path does not exist");
    let scene = importer.read_file_default(path);
    assert!(scene.is_ok(), "error: {}", scene.err().unwrap());
    scene.unwrap()
}

#[test]
fn test_dae_import_cube_scene_name() {
    let scene = load_cube_scene();
    assert_eq!(scene.name, "reportScene");
}

#[test]
fn test_dae_import_cube_materials() {
    let scene = load_cube_scene();
    assert_eq!(scene.materials.len(), 1);
}
