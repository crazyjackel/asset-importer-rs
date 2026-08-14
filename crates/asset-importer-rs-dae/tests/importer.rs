use std::{
    io::{self, Cursor},
    path::Path,
};

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_dae::{DaeImportError, DaeImporter};

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

#[test]
fn test_dae_read_file_not_implemented() {
    let importer = DaeImporter::new();
    let result = importer.read_file("model.dae", dummy_loader);
    assert!(matches!(result, Err(DaeImportError::NotImplemented)));
}
