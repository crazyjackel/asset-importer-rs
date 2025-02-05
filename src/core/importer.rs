#[derive(Debug)]
pub struct AiImporter {
    importer_scale: f64,
    file_scale: f64,
}

impl Default for AiImporter {
    fn default() -> Self {
        Self {
            importer_scale: 1.0,
            file_scale: 1.0,
        }
    }
}
