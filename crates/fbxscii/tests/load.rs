use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use fbxscii::{ElementAmphitheatre, Parser, Tokenizer};

/// Canonical ASCII duck used by `fbx-dom` (`crates/fbx-dom/assets/duck.fbx`).
fn duck_fbx_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fbx-dom/assets/duck.fbx")
}

fn count_element_key(arena: &ElementAmphitheatre, key: &str) -> usize {
    arena.iter().filter(|e| e.key == key).count()
}

fn first_element_with_key<'a>(arena: &'a ElementAmphitheatre, key: &str) -> Option<&'a fbxscii::Element> {
    arena.iter().find(|e| e.key == key)
}

#[test]
fn test_tokenizer_load_duck() {
    let path = duck_fbx_path();
    assert!(
        path.exists(),
        "duck.fbx missing at {} (expected fbx-dom test asset)",
        path.display()
    );
    let file = File::open(&path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let mut tokens = Vec::new();
    for token in tokenizer.flatten() {
        tokens.push(token);
    }
    assert_eq!(tokens.len(), 90627);
}

#[test]
fn test_parser_load_duck() {
    let path = duck_fbx_path();
    assert!(
        path.exists(),
        "duck.fbx missing at {} (expected fbx-dom test asset)",
        path.display()
    );
    let file = File::open(&path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let parser = Parser::new(tokenizer);
    let elements = parser.load().unwrap();
    assert_eq!(elements.count(), 517);
}

/// Parsed tree and headline tokens agree with `crates/fbx-dom/assets/duck.fbx` (FBX 7.3 ASCII).
#[test]
fn duck_fbx_fixture_semantics_match_fbx_dom_asset() {
    let path = duck_fbx_path();
    assert!(path.exists(), "duck.fbx missing at {}", path.display());

    let file = File::open(&path).unwrap();
    let tokenizer = Tokenizer::new(BufReader::new(file));
    let parser = Parser::new(tokenizer);
    let arena = parser.load().unwrap();

    let fbx_version = first_element_with_key(&arena, "FBXVersion").expect("FBXVersion element");
    assert_eq!(
        fbx_version.tokens.first().map(String::as_str),
        Some("7300"),
        "FBXHeaderExtension / FBXVersion should match ASCII header (7300)"
    );

    let creator = first_element_with_key(&arena, "Creator").expect("Creator element");
    let creator_str = creator.tokens.first().map(String::as_str).unwrap_or("");
    assert!(
        creator_str.contains("FBX SDK") && creator_str.contains("2013.1"),
        "Creator should match fixture: {creator_str:?}"
    );

    assert_eq!(
        count_element_key(&arena, "Model"),
        3,
        "Objects: three Model nodes (LOD3sp, camera1, directionalLight1)"
    );
    assert_eq!(count_element_key(&arena, "Geometry"), 1, "Objects: one Mesh geometry");
    assert_eq!(
        count_element_key(&arena, "Material"),
        1,
        "Objects: Material::blinn3"
    );
    assert_eq!(count_element_key(&arena, "Texture"), 1);
    assert_eq!(count_element_key(&arena, "Video"), 1);
    assert_eq!(count_element_key(&arena, "NodeAttribute"), 2);
    assert_eq!(count_element_key(&arena, "AnimationStack"), 1);
    assert_eq!(count_element_key(&arena, "AnimationLayer"), 1);

    // `get_handle_by_key` is ambiguous if multiple `Connections` nodes exist; assert the
    // document-level block’s child count (ten `C:` OO/OP rows in duck.fbx).
    let connection_row_counts: Vec<usize> = arena
        .iter()
        .enumerate()
        .filter(|(_, e)| e.key == "Connections")
        .filter_map(|(i, _)| arena.get_handle(i))
        .map(|h| h.children().filter(|ch| ch.key() == "C").count())
        .collect();
    assert!(
        connection_row_counts.iter().any(|&n| n == 10),
        "expected a Connections block with ten C rows, got per-block counts {connection_row_counts:?}"
    );
}
