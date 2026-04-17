use std::{fs::File, io::BufReader, path::Path};

use fbxscii::{Parser, Tokenizer};
use fbx_dom::{Document, ImportSettings, OwnedObject};

#[test]
fn test_load_duck_fbx() {
    let path = Path::new("assets/duck.fbx");
    assert!(path.exists(), "duck.fbx file does not exist at assets/duck.fbx");
    
    let file = File::open(path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let parser = Parser::new(tokenizer);
    
    let document = Document::from_parser(parser, ImportSettings::default());
    assert!(document.is_ok(), "Failed to load duck.fbx: {:?}", document.err());
    
    let document = document.unwrap();
    assert!(document.version() > 0, "FBX version should be set");
    assert!(!document.creator().is_empty(), "Creator should be set");

    let objects = document.objects();
    for result in objects {
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
    assert!(
        !document.creator().is_empty(),
        "Creator should be set"
    );

    let objects = document.objects();
    for result in objects {
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

