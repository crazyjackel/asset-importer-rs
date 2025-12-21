use fbxscii::{ElementAmphitheatre, ElementParseError, Parser, ParserError};
use std::{collections::HashMap, io::BufRead};

use crate::global::GlobalSettings;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq)]
pub enum PropertyParseError {
    InvalidTokenLength(usize, Option<String>),
    MissingPropertyType(String),
}

pub type Template = HashMap<String, Property>;

#[derive(Debug, PartialEq, Clone)]
pub struct LazyObject{
    pub name: String,
    pub type_name: String,
    pub class_name: String,
    /// Index of the equivalent element in the object_element_amphitheatre of the document
    /// Used for lazy loading of Type Specific Information
    pub element_index: usize
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct ObjectPropertyConnection{
    pub dest: u64,
    pub property: String,
}

#[derive(Default, Debug, Clone)]
pub struct Document {
    /// The version of the FBX file
    pub fbx_version: u32,
    /// The creating program of the FBX file
    pub creator: String,
    /// The creation date of the FBX file
    pub creation_date: [u32; 7],
    /// The templates of the FBX file
    pub templates: HashMap<String, Template>,
    /// The global settings of the FBX file
    pub global_settings: Template,
    /// The element amphitheatre containing object information 
    pub object_element_amphitheatre: ElementAmphitheatre,
    /// The objects of the FBX file
    pub objects: HashMap<u64, LazyObject>,
    /// The connections between objects
    pub object_connections: HashMap<u64, Vec<u64>>,
    /// The connections between object properties
    pub object_property_connections: HashMap<u64, Vec<ObjectPropertyConnection>>,
    /// The connections between properties
    pub property_connections: HashMap<ObjectPropertyConnection, Vec<ObjectPropertyConnection>>,
}

pub trait DocumentLoader {
    fn load_into_document(
        self,
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

    pub fn global_settings(&self) -> GlobalSettings<'_> {
        GlobalSettings::new(self, &self.global_settings)
    }
}
