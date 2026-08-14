use std::{
    io::{self, Cursor},
    path::Path,
};

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_dae::{DaeImportError, DaeImporter};

fn dummy_loader(_path: &Path) -> io::Result<Cursor<Vec<u8>>> {
    Ok(Cursor::new(Vec::new()))
}

#[test]
fn test_dae_can_read_extension() {
    let importer = DaeImporter::new();
    assert!(importer.can_read("model.dae", dummy_loader));
    assert!(!importer.can_read("model.obj", dummy_loader));
    assert!(!importer.can_read("model", dummy_loader));
}

#[test]
fn test_dae_read_file_not_implemented() {
    let importer = DaeImporter::new();
    let result = importer.read_file("model.dae", dummy_loader);
    assert!(matches!(result, Err(DaeImportError::NotImplemented)));
}
