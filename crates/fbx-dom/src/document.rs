use fbxscii::{ElementArena, ElementParseError, Parser, ParserError};
use fbxcel::tree::v7400::Tree;
use std::io::BufRead;

#[derive(Debug)]
pub enum DocumentParseError {
    ParserError(ParserError),
    UnsupportedVersion(u32),
    RequiredElementNotFound(String),
    ElementParseError(ElementParseError),
}

#[derive(Default)]
pub struct Document {
    fbx_version: u32,
    creator: String,
    creation_date: [u32; 7]
}

trait HeaderReader {
    fn read_header(&self, document: &mut Document) -> Result<(), DocumentParseError>;
}

impl Document {
    pub fn from_parser<R>(parser: Parser<R>) -> Result<Self, DocumentParseError>
    where
        R: BufRead,
    {
        let elements = parser.load().map_err(DocumentParseError::ParserError)?;
        let mut document = Self::default();
        elements.read_header(&mut document)?;
        Ok(document)
    }
}

impl HeaderReader for ElementArena {

    fn read_header(&self, document: &mut Document) -> Result<(), DocumentParseError> {
        let header_extension = self.get_handle_by_key("FBXHeaderExtension");
        if header_extension.is_none() {
            return Err(DocumentParseError::RequiredElementNotFound("FBXHeaderExtension".to_string()));
        }
        let header_extension = header_extension.unwrap();
        let version_element = header_extension.first_child_by_key("FBXVersion").ok_or(DocumentParseError::RequiredElementNotFound("FBXVersion".to_string()))?;
        let version: u32 = version_element.try_into().map_err(DocumentParseError::ElementParseError)?;
        document.fbx_version = version;

        Ok(())
    }
}