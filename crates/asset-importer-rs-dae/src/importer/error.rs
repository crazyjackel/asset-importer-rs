use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum DaeImportError {
    NotImplemented,
}

impl Display for DaeImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaeImportError::NotImplemented => {
                write!(f, "DAE import is not implemented")
            }
        }
    }
}

impl Error for DaeImportError {}
