use std::{fs::File, io::BufReader, path::Path};

use fbx_dom::{
    Document, ImportSettings, ModelGeometryRef, NodeAttributeRef, OwnedDocument, OwnedObject,
    Property,
};
use fbxscii::{Parser, Tokenizer};

#[test]
fn test_load_duck_fbx() {
    let path = Path::new("assets/duck.fbx");
    assert!(
        path.exists(),
        "duck.fbx file does not exist at assets/duck.fbx"
    );

    let file = File::open(path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let parser = Parser::new(tokenizer);

    let document = Document::from_parser(parser, ImportSettings::default());
    assert!(
        document.is_ok(),
        "Failed to load duck.fbx: {:?}",
        document.as_ref().err()
    );

    let document = document.unwrap();

    assert_eq!(document.version(), 7300, "FBXVersion in FBXHeaderExtension");
    assert!(
        document.creator().contains("FBX SDK/FBX Plugins"),
        "Creator string: {}",
        document.creator()
    );
    assert!(
        document.creator().contains("2013.1"),
        "Creator should mention plugin year"
    );
    assert_eq!(
        document.creation_date(),
        &[2012, 6, 28, 16, 32, 53, 433],
        "CreationTimeStamp in duck.fbx header"
    );

    let globals = document.global_settings();
    assert_eq!(globals.up_axis(), 1);
    assert_eq!(globals.up_axis_sign(), 1);
    assert_eq!(globals.front_axis(), 2);
    assert_eq!(globals.unit_scale_factor(), 1.0);
    assert_eq!(
        globals.default_camera(),
        "Producer Perspective",
        "DefaultCamera in GlobalSettings"
    );
    let time_span_stop_ok = match globals.global_settings().get("TimeSpanStop") {
        Some(Property::ULongLong(v)) => *v == 46186158000,
        Some(Property::ILongLong(v)) => *v == 46186158000,
        _ => false,
    };
    assert!(
        time_span_stop_ok,
        "TimeSpanStop should be 46186158000 (KTime), got {:?}",
        globals.global_settings().get("TimeSpanStop")
    );

    let object_count = document.objects().filter(|r| r.is_ok()).count();
    assert_eq!(object_count, 11, "Objects section row count for duck.fbx");

    let lod3 = document
        .object_by_index(40530896)
        .expect("Model::LOD3sp id from duck.fbx");
    assert_eq!(lod3.name(), "Model::LOD3sp");
    assert_eq!(lod3.type_name(), "Model");
    assert_eq!(lod3.class_name(), "Mesh");

    let material = document
        .object_by_index(39551424)
        .expect("Material::blinn3 id from duck.fbx");
    assert_eq!(material.name(), "Material::blinn3");
    assert!(
        material.connected_object_ids().contains(&40530896),
        "OO: Material -> Model::LOD3sp"
    );

    let geometry = document
        .object_by_index(40533296)
        .expect("Geometry::LOD3spShape id from duck.fbx");
    assert_eq!(geometry.name(), "Geometry::LOD3spShape");
    assert!(
        geometry.connected_object_ids().contains(&40530896),
        "OO: Geometry -> Model::LOD3sp"
    );

    let texture = document
        .object_by_index(40532784)
        .expect("Texture::file2 id from duck.fbx");
    assert_eq!(texture.name(), "Texture::file2");
    let op_targets = texture.object_property_targets();
    assert_eq!(op_targets.len(), 1);
    assert_eq!(op_targets[0].dest, 39551424);
    assert_eq!(op_targets[0].property, "DiffuseColor");

    let video = document
        .object_by_index(39875536)
        .expect("Video::file2 id from duck.fbx");
    assert_eq!(video.name(), "Video::file2");
    assert!(
        video.connected_object_ids().contains(&40532784),
        "OO: Video -> Texture::file2"
    );

    for result in document.objects() {
        if let Ok(object) = result {
            let owned_object: OwnedObject = object.into();
            println!("Object: {}", owned_object.name);
            println!("Type Name: {}", owned_object.type_name);
            println!("Class Name: {}", owned_object.class_name);
            println!("Properties: {:?}", owned_object.properties);
            println!("Attributes: {:?}", owned_object.attributes);
        }
    }
}

fn owned_duck_row_count(owned: &OwnedDocument) -> usize {
    owned.models.len()
        + owned.mesh_geometries.len()
        + owned.line_geometries.len()
        + owned.shape_geometries.len()
        + owned.unknown_geometries.len()
        + owned.cameras.len()
        + owned.camera_switchers.len()
        + owned.lights.len()
        + owned.null_nodes.len()
        + owned.limb_nodes.len()
        + owned.unknown_node_attributes.len()
        + owned.materials.len()
        + owned.textures.len()
        + owned.layered_textures.len()
        + owned.videos.len()
        + owned.clusters.len()
        + owned.skins.len()
        + owned.blend_shapes.len()
        + owned.blend_shape_channels.len()
        + owned.unknown_deformers.len()
        + owned.animation_stacks.len()
        + owned.animation_layers.len()
        + owned.animation_curves.len()
        + owned.animation_curve_nodes.len()
        + owned.unknown_objects.len()
}

#[test]
fn duck_owned_document_from_ascii() {
    let path = Path::new("assets/duck.fbx");
    assert!(path.exists(), "duck.fbx missing at assets/duck.fbx");

    let file = File::open(path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let parser = Parser::new(tokenizer);
    let document = Document::from_parser(parser, ImportSettings::default()).unwrap();
    let owned = OwnedDocument::from(document);

    assert_eq!(owned.fbx_version, 7300);
    assert!(owned.creator.contains("FBX SDK/FBX Plugins"));
    assert_eq!(
        owned.creation_date,
        [2012, 6, 28, 16, 32, 53, 433],
        "header carried into OwnedDocument"
    );

    assert_eq!(
        owned_duck_row_count(&owned),
        11,
        "every Objects row is classified into exactly one OwnedDocument bucket"
    );
    assert_eq!(owned.models.len(), 3);

    let lod3 = owned
        .models
        .iter()
        .find(|m| m.inner().object_index == 40530896)
        .expect("Model::LOD3sp");
    assert_eq!(lod3.inner().name, "Model::LOD3sp");

    // When Material / Texture / MeshGeometry narrow successfully, OO/OP helpers match `test_load_duck_fbx`.
    if !owned.materials.is_empty() {
        let mats = lod3.connected_materials(&owned);
        assert_eq!(mats.len(), 1);
        assert_eq!(mats[0].inner().object_index, 39551424);
        if !owned.textures.is_empty() {
            let tex = mats[0].get_textures(&owned);
            assert_eq!(
                tex.get("DiffuseColor").map(|t| t.inner().object_index),
                Some(40532784)
            );
        }
    }

    let mesh_id = 40533296_u64;
    if !owned.mesh_geometries.is_empty() || !owned.unknown_geometries.is_empty() {
        let geos = lod3.connected_geometries(&owned);
        if owned
            .mesh_geometries
            .iter()
            .any(|g| g.inner().object_index == mesh_id)
            || owned
                .unknown_geometries
                .iter()
                .any(|o| o.object_index == mesh_id)
        {
            assert_eq!(geos.len(), 1);
            assert!(matches!(
                geos[0],
                ModelGeometryRef::Mesh(_) | ModelGeometryRef::Unknown(_)
            ));
            assert_eq!(geos[0].inner().object_index, mesh_id);
        }
    }

    assert!(
        lod3.connected_node_attributes(&owned).is_empty(),
        "LOD3 mesh model has no NodeAttribute OO parents in duck.fbx"
    );

    if !owned.cameras.is_empty() {
        let cam_model = owned
            .models
            .iter()
            .find(|m| m.inner().object_index == 39982240)
            .expect("Model::camera1");
        let cam_attrs = cam_model.connected_node_attributes(&owned);
        assert_eq!(cam_attrs.len(), 1);
        assert!(matches!(cam_attrs[0], NodeAttributeRef::Camera(_)));
        assert_eq!(cam_attrs[0].inner().object_index, 39870592);
    }

    if !owned.lights.is_empty() {
        let light_model = owned
            .models
            .iter()
            .find(|m| m.inner().object_index == 39872528)
            .expect("Model::directionalLight1");
        let light_attrs = light_model.connected_node_attributes(&owned);
        assert_eq!(light_attrs.len(), 1);
        assert!(matches!(light_attrs[0], NodeAttributeRef::Light(_)));
        assert_eq!(light_attrs[0].inner().object_index, 39984128);
    }
}

/// Binary FBX from [Kenney Starter Kit Racing](https://github.com/KenneyNL/Starter-Kit-Racing/blob/main/models/collision-track-straight.fbx).
#[test]
fn test_load_collision_track_straight_fbx() {
    let path = Path::new("assets/collision-track-straight.fbx");
    assert!(
        path.exists(),
        "collision-track-straight.fbx missing at assets/collision-track-straight.fbx (add from Kenney Starter-Kit-Racing)"
    );

    let file = File::open(path).unwrap();
    let document = Document::from_binary_reader(file, ImportSettings::default());
    assert!(
        document.is_ok(),
        "Failed to load collision-track-straight.fbx: {:?}",
        document.as_ref().err()
    );

    let document = document.unwrap();
    assert!(document.version() > 0, "FBX version should be set");
    assert!(!document.creator().is_empty(), "Creator should be set");

    let loadable_rows = document.objects().filter(|r| r.is_ok()).count();
    assert!(
        loadable_rows > 0,
        "expected at least one Objects row with a template"
    );

    let owned = OwnedDocument::from(document);

    assert!(
        owned.fbx_version > 0,
        "header version carried into OwnedDocument"
    );
    assert!(
        !owned.creator.is_empty(),
        "creator copied into OwnedDocument"
    );

    assert_eq!(
        owned_duck_row_count(&owned),
        loadable_rows,
        "each template-resolved Object row lands in exactly one OwnedDocument bucket"
    );

    assert!(
        !owned.models.is_empty() || !owned.mesh_geometries.is_empty(),
        "Kenney track asset should include at least models or mesh geometries"
    );
}
