use std::{error::Error, fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum DaeImportError {
    FileOpenError(io::Error, PathBuf),
    FileFormatError(dae_parser::Error),
    MissingVisualScene,
}

impl Display for DaeImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaeImportError::FileOpenError(error, path) => {
                write!(f, "failed to open DAE file '{}': {}", path.display(), error)
            }
            DaeImportError::FileFormatError(error) => {
                write!(f, "failed to parse DAE file: {:?}", error)
            }
            DaeImportError::MissingVisualScene => {
                write!(f, "no visual scene found in DAE file")
            }
        }
    }
}

impl Error for DaeImportError {}
