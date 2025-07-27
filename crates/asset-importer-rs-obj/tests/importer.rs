use std::path::Path;

use asset_importer_rs_core::AiImporterExt;
use asset_importer_rs_obj::ObjImporter;

#[test]
fn test_importer_obj() {
    let importer = ObjImporter::new();
    let path = Path::new("assets/spider.obj");
    assert!(path.exists(), "path does not exist");
    let scene = importer.read_file_default(path);
    assert!(scene.is_ok(), "error: {}", scene.err().unwrap());
    assert_eq!(scene.unwrap().name, "spider");
}
