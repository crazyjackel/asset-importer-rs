use std::{fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum DaeImportError {
    FileOpenError(io::Error, PathBuf),
    FileFormatError(dae_parser::Error),
    MissingLocalMapEntry(String),
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
            DaeImportError::MissingLocalMapEntry(url) => {
                write!(f, "no local map entry found for URL: {}", url)
            }
            DaeImportError::MissingVisualScene => {
                write!(f, "no visual scene found in DAE file")
            }
        }
    }
}

impl std::error::Error for DaeImportError {}
