use std::result;

use crate::binary;

/// Result type for convenience.
pub type Result<T> = result::Result<T, Error>;

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    Base64(base64::DecodeError),
    BufferLength {
        buffer: String,
        expected: usize,
        actual: usize,
    },
    Deserialize(json::Error),
    Io(std::io::Error),
    Image(image_crate::ImageError),
    Validation(Vec<(json::Path, json::validation::Error)>),
    Binary(binary::Error),
    ExternalReferenceInSliceImport,
    UnsupportedScheme,
    MissingBlob,
    UnsupportedImageEncoding,
    UnsupportedImageFormat(image_crate::DynamicImage),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Deserialize(e) => e.fmt(f),
            Error::Binary(e) => e.fmt(f),
            Error::Validation(ref xs) => {
                write!(f, "invalid glTF 1.0:")?;
                for (ref path, ref error) in xs {
                    write!(f, " {}: {};", path, error)?;
                }
                Ok(())
            }
            Error::Base64(ref e) => e.fmt(f),
            Error::ExternalReferenceInSliceImport => {
                write!(f, "external reference in slice only import")
            }
            Error::UnsupportedScheme => write!(f, "unsupported URI scheme"),
            Error::MissingBlob => write!(f, "missing binary portion of binary glTF"),
            Error::BufferLength {
                buffer,
                expected,
                actual,
            } => write!(
                f,
                "buffer {}: expected {} bytes but received {} bytes",
                buffer, expected, actual
            ),
            Error::UnsupportedImageEncoding => write!(f, "unsupported image encoding"),
            Error::UnsupportedImageFormat(image) => {
                write!(f, "unsupported image format: {:?}", image.color())
            }
            Error::Image(ref e) => e.fmt(f),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<json::Error> for Error {
    fn from(value: json::Error) -> Self {
        Error::Deserialize(value)
    }
}
impl From<binary::Error> for Error {
    fn from(err: binary::Error) -> Self {
        Error::Binary(err)
    }
}
impl From<image_crate::ImageError> for Error {
    fn from(value: image_crate::ImageError) -> Self {
        Error::Image(value)
    }
}
