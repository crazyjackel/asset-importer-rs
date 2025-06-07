use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    MissingBufferData,
    ExceedsBounds,
    DuplicateName,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingBufferData => write!(f, "Missing Buffer Data"),
            Error::ExceedsBounds => write!(f, "Exceeds Bounds"),
            Error::DuplicateName => write!(f, "Duplicate Name"),
        }
    }
}

impl std::error::Error for Error {}
