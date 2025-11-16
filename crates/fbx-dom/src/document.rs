use fbxscii::{ElementParseError, Parser, ParserError};
use std::{collections::HashMap, io::BufRead};

#[derive(Debug)]
pub enum DocumentParseError {
    ParserError(ParserError),
    UnsupportedVersion(u32, Option<String>),
    RequiredElementNotFound(String),
    ElementParseError(ElementParseError),
    PropertyParseError(PropertyParseError),
}

#[derive(Debug, Default)]
pub struct ImportSettings {
    pub strict: bool,
}

#[derive(Debug)]
pub enum Property{
    String(String),
    Bool(bool),
    Int(i32),
    Float(f32),
    ULongLong(u64),
    ILongLong(i64),
    Vec3([f32; 3]),
    Vec4([f32; 4])
}

#[derive(Debug)]
pub struct PropertyDetails{
    pub name: String,
    pub property: Property,
}

#[derive(Debug)]
pub enum PropertyParseError {
    InvalidTokenLength(usize, Option<String>),
    MissingPropertyType(String),
}

// @todo: Consider Lazy Loading of Properties
pub type Template = HashMap<String, Property>;

#[derive(Default)]
pub struct Document {
    pub fbx_version: u32,
    pub creator: String,
    pub creation_date: [u32; 7],
    pub templates: HashMap<String, Template>
}

pub trait DocumentLoader {
    fn load_into_document(
        &self,
        document: &mut Document,
        settings: ImportSettings,
    ) -> Result<(), DocumentParseError>;
}

impl Document {
    pub fn from_parser<R>(
        parser: Parser<R>,
        settings: ImportSettings,
    ) -> Result<Self, DocumentParseError>
    where
        R: BufRead,
    {
        let elements = parser.load().map_err(DocumentParseError::ParserError)?;
        let mut document = Self::default();
        elements.load_into_document(&mut document, settings)?;
        Ok(document)
    }
}
