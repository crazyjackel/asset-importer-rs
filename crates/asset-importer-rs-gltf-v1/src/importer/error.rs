use std::{fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum GLTFImportError {
    MissingBufferData,
    ExceedsBounds,
    DuplicateName,
    FileOpenError(io::Error, PathBuf),
    FileFormatError(gltf_v1::GLTF_Error),
}
impl Display for GLTFImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GLTFImportError::MissingBufferData => write!(f, "Missing Buffer Data"),
            GLTFImportError::ExceedsBounds => write!(f, "Exceeds Bounds"),
            GLTFImportError::DuplicateName => write!(f, "Duplicate Name"),
            GLTFImportError::FileOpenError(error, path_buf) => {
                write!(
                    f,
                    "Failed to open file: '{}': {}",
                    path_buf.display(),
                    error
                )
            }
            GLTFImportError::FileFormatError(error) => write!(f, "File Format Error: {}", error),
        }
    }
}

impl std::error::Error for GLTFImportError {}
