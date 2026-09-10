use std::{
    io::{self, Cursor},
    path::Path,
};

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_dae::{
    AI_COLLADA_AUTHOR, AI_COLLADA_COMMENTS, AI_COLLADA_CREATED, AI_COLLADA_KEYWORDS,
    AI_COLLADA_MODIFIED, AI_COLLADA_REVISION, AI_COLLADA_SOURCE_DATA, AI_COLLADA_SUBJECT,
    AI_COLLADA_TITLE, AI_METADATA_SOURCE_COPYRIGHT, AI_METADATA_SOURCE_GENERATOR, DaeImportError,
    DaeImporter,
};
use asset_importer_rs_scene::{
    AiColor4D, AiMatrix4x4, AiMetadataEntry, AiShadingMode, AiTextureType,
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
fn test_dae_import_cube_cameras() {
    let scene = load_cube_scene();
    assert!(scene.cameras.is_empty());
}

#[test]
fn test_dae_import_cube_lights() {
    let scene = load_cube_scene();
    assert!(scene.lights.is_empty());
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

fn node_only_dae(unit_meter: &str, up_axis: &str) -> Vec<u8> {
    format!(
        r##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <created>1970-01-01T00:00:00Z</created>
    <modified>1970-01-01T00:00:00Z</modified>
    <unit meter="{unit_meter}"/>
    <up_axis>{up_axis}</up_axis>
  </asset>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node id="Root"/>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>
</COLLADA>"##
    )
    .into_bytes()
}

#[test]
fn test_dae_applies_unit_size_and_z_up() {
    let importer = DaeImporter::new();
    let xml = node_only_dae("2", "Z_UP");
    let scene = importer
        .read_file("unit-up.dae", |_| Ok(Cursor::new(xml.clone())))
        .expect("import");
    let root = scene.nodes.root.expect("root");
    let expected = {
        let mut m = AiMatrix4x4::identity();
        m *= AiMatrix4x4::from([
            2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        m *= AiMatrix4x4::from([
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        m
    };
    assert_eq!(scene.nodes.arena[root].transformation, expected);
}

#[test]
fn test_dae_ignore_unit_size_and_up_direction() {
    let importer = DaeImporter {
        ignore_unit_size: true,
        ignore_up_direction: true,
        ..DaeImporter::new()
    };
    let xml = node_only_dae("2", "Z_UP");
    let scene = importer
        .read_file("ignore-unit-up.dae", |_| Ok(Cursor::new(xml.clone())))
        .expect("import");
    let root = scene.nodes.root.expect("root");
    assert_eq!(
        scene.nodes.arena[root].transformation,
        AiMatrix4x4::identity()
    );
}

fn metadata_str<'a>(
    metadata: &'a asset_importer_rs_scene::AiMetadata,
    key: &str,
) -> &'a str {
    match metadata.get(key) {
        Some(AiMetadataEntry::AiStr(s)) => s,
        other => panic!("expected string metadata for {key}, got {other:?}"),
    }
}

#[test]
fn test_dae_import_cube_metadata() {
    let scene = load_cube_scene();
    assert_eq!(
        metadata_str(&scene.metadata, AI_METADATA_SOURCE_GENERATOR),
        "SceneKit Collada Exporter v1.0"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_CREATED),
        "2018-10-25T16:29:03+00:00"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_MODIFIED),
        "2018-10-25T16:29:03+00:00"
    );
}

#[test]
fn test_dae_import_asset_metadata_fields() {
    let importer = DaeImporter::new();
    let xml = br##"<?xml version="1.0"?>
<COLLADA xmlns="http://www.collada.org/2005/11/COLLADASchema" version="1.4.1">
  <asset>
    <contributor>
      <author>First Author</author>
      <authoring_tool>Tool A</authoring_tool>
      <comments>First comments</comments>
      <copyright>Copyright A</copyright>
      <source_data>file:///first.blend</source_data>
    </contributor>
    <contributor>
      <author>Second Author</author>
      <authoring_tool>Tool B</authoring_tool>
      <comments>Second comments</comments>
      <copyright>Copyright B</copyright>
      <source_data>file:///second.blend</source_data>
    </contributor>
    <created>2018-10-25T16:29:03Z</created>
    <keywords>alpha beta gamma</keywords>
    <modified>2018-10-26T00:00:00</modified>
    <revision>3</revision>
    <subject>Test subject</subject>
    <title>Test title</title>
  </asset>
  <library_visual_scenes>
    <visual_scene id="Scene">
      <node id="Root"/>
    </visual_scene>
  </library_visual_scenes>
  <scene>
    <instance_visual_scene url="#Scene"/>
  </scene>
</COLLADA>"##;
    let scene = importer
        .read_file("metadata.dae", |_| Ok(Cursor::new(xml.to_vec())))
        .expect("import");

    assert_eq!(
        metadata_str(&scene.metadata, AI_METADATA_SOURCE_GENERATOR),
        "Tool A"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_METADATA_SOURCE_COPYRIGHT),
        "Copyright A"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_AUTHOR),
        "First Author"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_COMMENTS),
        "First comments"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_SOURCE_DATA),
        "file:///first.blend"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_CREATED),
        "2018-10-25T16:29:03+00:00"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_MODIFIED),
        "2018-10-26T00:00:00"
    );
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_KEYWORDS),
        "alpha beta gamma"
    );
    assert_eq!(metadata_str(&scene.metadata, AI_COLLADA_REVISION), "3");
    assert_eq!(
        metadata_str(&scene.metadata, AI_COLLADA_SUBJECT),
        "Test subject"
    );
    assert_eq!(metadata_str(&scene.metadata, AI_COLLADA_TITLE), "Test title");
}
