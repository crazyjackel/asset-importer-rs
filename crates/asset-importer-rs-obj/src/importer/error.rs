use std::{error::Error, fmt::Display, io, path::PathBuf};

use image::ImageError;

#[derive(Debug)]
pub enum ObjImportError {
    FileOpenError(io::Error, PathBuf),
    FileReadError(io::Error),
    ImageLoadError(ImageError),
    UnsupportedFileFormat(ImageError, String),
    ObjLoadError(tobj::LoadError),
    MtlLoadError(tobj::LoadError),
}

impl Display for ObjImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjImportError::FileOpenError(error, path) => {
                write!(f, "failed to open OBJ file '{}': {}", path.display(), error)
            }
            ObjImportError::UnsupportedFileFormat(error, format) => {
                write!(f, "unsupported file format: '{}': {}", format, error)
            }
            ObjImportError::FileReadError(error) => {
                write!(f, "failed to read OBJ file: {}", error)
            }
            ObjImportError::ImageLoadError(image_error) => {
                write!(f, "failed to load image: {}", image_error)
            }
            ObjImportError::ObjLoadError(load_error) => {
                write!(f, "failed to load OBJ file: {}", load_error)
            }
            ObjImportError::MtlLoadError(load_error) => {
                write!(f, "failed to load MTL file: {}", load_error)
            }
        }
    }
}

impl Error for ObjImportError {}
