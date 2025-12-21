use std::{fs::File, io::BufReader, path::Path};

use fbxscii::{Parser, Tokenizer};
use fbx_dom::document::{Document, ImportSettings};

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
    assert!(document.fbx_version > 0, "FBX version should be set");
    assert!(!document.creator.is_empty(), "Creator should be set");
    assert!(!document.objects.is_empty(), "Document should contain objects");
}

