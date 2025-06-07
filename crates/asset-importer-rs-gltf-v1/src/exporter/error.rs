use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    MissingMaterial,
    UTFError(FromUtf8Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingMaterial => write!(f, "Missing Material"),
            Error::UTFError(error) => write!(f, "UTF Error: {}", error),
        }
    }
}

impl std::error::Error for Error {}
