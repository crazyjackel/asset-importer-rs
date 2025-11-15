use std::{fs::File, io::BufReader, path::Path};

use fbxscii::Tokenizer;

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
