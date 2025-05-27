use asset_importer_rs::AssetImporter;

#[test]
fn test_importer_gltf2() {
    let scene = AssetImporter::from_file("tests/model/gltf2/Avocado.glb");
    assert!(scene.is_ok());
    assert_eq!(scene.unwrap().name, "");
}
