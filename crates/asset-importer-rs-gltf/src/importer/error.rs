use std::{error::Error, fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum MeshError {
    ExceedsBounds,
    InvalidStride,
    SizeExceedsTarget,
    MissingBufferData,
    BrokenSparseDataAccess,
    AttributeNotFound,
}

impl Display for MeshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeshError::MissingBufferData => write!(f, "Missing Buffer Data"),
            MeshError::BrokenSparseDataAccess => {
                write!(f, "Sparse Data Missing despite being expected")
            }
            MeshError::ExceedsBounds => write!(f, "Bounds Check Failed"),
            MeshError::AttributeNotFound => {
                write!(f, "Expected Primitive Attribute Not Found")
            }
            MeshError::SizeExceedsTarget => write!(f, "Size provided exceeds Target"),
            MeshError::InvalidStride => write!(f, "Stride is Less than Element Size"),
        }
    }
}

impl Error for MeshError {}

#[derive(Debug)]
pub enum Gltf2ImportError {
    FileOpenError(io::Error, PathBuf),
    FileFormatError(gltf::Error),
    MeshError(MeshError),
}

impl Display for Gltf2ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gltf2ImportError::MeshError(error) => write!(f, "Mesh Error: {}", error),
            Gltf2ImportError::FileOpenError(error, path) => {
                write!(f, "File Open Error: '{}': {}", path.display(), error)
            }
            Gltf2ImportError::FileFormatError(error) => {
                write!(f, "File Format Error: {}", error)
            }
        }
    }
}

impl Error for Gltf2ImportError {}
