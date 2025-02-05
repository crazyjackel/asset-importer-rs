use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub enum AiReadError{
    FileOpenError(Box<dyn std::error::Error>),
    FileFormatError(Box<dyn std::error::Error>),
    UnsupportedImageFormat(String,String)
}

impl Display for AiReadError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiReadError::FileOpenError(error) => write!(f, "Asset Importer File Open Error: {}", error),
            AiReadError::FileFormatError(error) => write!(f, "Asset Importer File Format Error: {}", error),
            AiReadError::UnsupportedImageFormat(format, image_format) => write!(f, "Asset Importer does not support Image Format ({}) for Format ({}) at this time. Let me know why this is crazy in Github Issues and I will get back to you in... like a year.", image_format, format),
        }
    }
}

impl Error for AiReadError{}

#[derive(Debug)]
pub enum AiExportError{
    FileWriteError(Box<dyn std::error::Error>),
    ConversionError(Box<dyn std::error::Error>)
}

impl Display for AiExportError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            AiExportError::FileWriteError(error) => write!(f, "Asset Importer File Write Error: {}", error),
            AiExportError::ConversionError(error) => write!(f, "Asset Importer Conversion Error: {}", error),
        }
    }
}
impl Error for AiExportError{}