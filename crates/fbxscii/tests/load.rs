use std::{fs::File, io::BufReader, path::Path};

use fbxscii::{Parser, Tokenizer};

#[test]
fn test_tokenizer_load_duck() {
    let path = Path::new("assets/duck.fbx");
    assert!(path.exists(), "path does not exist");
    let file = File::open(path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let mut tokens = Vec::new();
    for token in tokenizer.flatten() {
        println!("{:?}", token);
        tokens.push(token);
    }
    assert_eq!(tokens.len(), 90627);
}

#[test]
fn test_parser_load_duck() {
    let path = Path::new("assets/duck.fbx");
    assert!(path.exists(), "path does not exist");
    let file = File::open(path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let parser = Parser::new(tokenizer);
    let elements = parser.load().unwrap();
    assert_eq!(elements.root_index, 0);
}