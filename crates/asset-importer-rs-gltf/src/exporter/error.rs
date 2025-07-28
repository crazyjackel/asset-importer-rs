use std::{error::Error, fmt::Display, io, num::TryFromIntError, string::FromUtf8Error};

#[derive(Debug)]
pub enum Gltf2ExportError {
    FileOpen(io::Error),
    Json(serde_json::Error),
    Conversion(FromUtf8Error),
    IntConversion(TryFromIntError),
    FileFormat(gltf::Error),
}

impl Display for Gltf2ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gltf2ExportError::FileOpen(error) => {
                write!(f, "Gltf2ExportError: {}", error)
            }
            Gltf2ExportError::Conversion(error) => {
                write!(f, "Gltf2ExportError: {}", error)
            }
            Gltf2ExportError::Json(error) => {
                write!(f, "Gltf2ExportError: {}", error)
            }
            Gltf2ExportError::IntConversion(try_from_int_error) => {
                write!(f, "Gltf2ExportError: {}", try_from_int_error)
            }
            Gltf2ExportError::FileFormat(error) => {
                write!(f, "Gltf2ExportError: {}", error)
            }
        }
    }
}

impl Error for Gltf2ExportError {}
