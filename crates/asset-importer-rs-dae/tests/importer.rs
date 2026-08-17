use std::{
    io::{self, Cursor},
    path::Path,
};

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_dae::{DaeImportError, DaeImporter};
use asset_importer_rs_scene::{
    AiColor4D, AiShadingMode, AiTextureType,
    matkey::{
        AI_MATKEY_COLOR_DIFFUSE, AI_MATKEY_NAME, AI_MATKEY_OPACITY, AI_MATKEY_SHADING_MODEL,
        AI_MATKEY_SHININESS,
    },
};

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
    let material = &scene.materials[0];
    let name = material
        .get_property_ai_str(AI_MATKEY_NAME, Some(AiTextureType::None), 0)
        .unwrap()
        .unwrap();
    assert_eq!(name, "Blue");
    assert_eq!(
        material.get_property_byte(AI_MATKEY_SHADING_MODEL, Some(AiTextureType::None), 0),
        Some(AiShadingMode::Phong as u8)
    );
    let diffuse = material
        .get_property_ai_color_rgba(AI_MATKEY_COLOR_DIFFUSE, Some(AiTextureType::None), 0)
        .unwrap();
    assert_eq!(diffuse, AiColor4D::new(0.137255, 0.403922, 0.870588, 1.0));
    assert_eq!(
        material.get_property_ai_float(AI_MATKEY_SHININESS, Some(AiTextureType::None), 0),
        Some(16.0)
    );
    assert_eq!(
        material.get_property_ai_float(AI_MATKEY_OPACITY, Some(AiTextureType::None), 0),
        Some(1.0)
    );
}

#[test]
fn test_dae_import_cube_nodes() {
    let scene = load_cube_scene();
    let root = scene.nodes.root.expect("scene should have a root node");
    let root_node = &scene.nodes.arena[root];
    assert_eq!(root_node.name, "F1");
    assert!(root_node.children.is_empty());
    assert_eq!(root_node.parent, None);
}

#[test]
fn test_dae_import_cube_meshes() {
    let scene = load_cube_scene();
    assert_eq!(scene.meshes.len(), 1);
    let mesh = &scene.meshes[0];
    assert_eq!(mesh.name, "F1");
    assert_eq!(mesh.vertices.len(), 36);
    assert_eq!(mesh.faces.len(), 12);
    assert_eq!(mesh.material_index, 0);
    let root = scene.nodes.root.expect("scene should have a root node");
    assert_eq!(scene.nodes.arena[root].mesh_indexes, vec![0]);
}

#[test]
fn test_dae_import_empty_visual_scene_missing_root() {
    let importer = DaeImporter::new();
    let xml = br##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
  </asset>
  <library_visual_scenes>
    <visual_scene id="Empty"/>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Empty"/>
  </scene>
</COLLADA>"##;
    let result = importer.read_file("empty.dae", |_| Ok(Cursor::new(xml.to_vec())));
    assert!(matches!(result, Err(DaeImportError::MissingRootNode)));
}
