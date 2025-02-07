use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Gtlf2Error {
    ExceedsBounds,
    MissingBufferData,
    BrokenSparseDataAccess,
    AttributeNotFound,
}

impl Display for Gtlf2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gtlf2Error::MissingBufferData => write!(f, "Missing Buffer Data"),
            Gtlf2Error::BrokenSparseDataAccess => {
                write!(f, "Sparse Data Missing despite being expected")
            }
            Gtlf2Error::ExceedsBounds => write!(f, "Calculated Bounds exceeds Loaded Bounds"),
            Gtlf2Error::AttributeNotFound => {
                write!(f, "Expected Primitive Attribute Not Found")
            }
        }
    }
}

impl Error for Gtlf2Error {}
