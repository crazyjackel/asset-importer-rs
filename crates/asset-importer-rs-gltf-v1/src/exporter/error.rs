use std::{fmt::Display, io, string::FromUtf8Error};

#[derive(Debug)]
pub enum GltfExportError {
    MissingMaterial,
    UTFError(FromUtf8Error),
    Io(io::Error),
    Json(serde_json::Error),
    FileFormat(gltf_v1::GLTF_Error),
}
impl Display for GltfExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GltfExportError::MissingMaterial => write!(f, "Missing Material"),
            GltfExportError::UTFError(error) => write!(f, "UTF Error: {}", error),
            GltfExportError::Io(error) => write!(f, "IO Error: {}", error),
            GltfExportError::Json(error) => write!(f, "JSON Error: {}", error),
            GltfExportError::FileFormat(error) => write!(f, "File Format Error: {}", error),
        }
    }
}

impl std::error::Error for GltfExportError {}
