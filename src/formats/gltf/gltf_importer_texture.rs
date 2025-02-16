use std::path::Path;

use gltf_v1::{buffer, document::Document, json::map::IndexMap};

use super::gltf_importer::GltfImporter;

impl GltfImporter {
    pub(crate) fn import_embedded_textures(
        document: &Document,
        base: Option<&Path>,
        buffer_data: &IndexMap<String, buffer::Data>,
    ) -> Result<(Vec<AiTexture>, HashMap<usize, usize>), AiReadError> {
        document.te
    }
}

#[test]
fn test_gltf_texture_import() {
    let binding = std::env::current_dir().expect("Failed to get the current executable path");
    let exe_path = binding.as_path();

    let gltf_data = r#"{
            "asset": {
                "version": "2.0"
            },
            "images": [
                {
                    "uri": "tests/textures/Unicode❤♻Texture.png"
                }
            ],
            "textures": [
                {
                    "source": 0
                }
            ]
        }"#;
    let scene = serde_json::from_str(gltf_data).unwrap();
    let document = Document::from_json_without_validation(scene);
    let (embedded_textures, _tex_ids) =
        GltfImporter::import_embedded_textures(&document, Some(exe_path), &IndexMap::new())
            .unwrap();
    assert_eq!(1, embedded_textures.len());
}
