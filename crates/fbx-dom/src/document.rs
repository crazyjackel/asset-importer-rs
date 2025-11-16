use fbxcel::tree::v7400::Tree;
use fbxscii::{ElementAmphitheatre, ElementParseError, Parser, ParserError};
use std::io::BufRead;

const LOWEST_SUPPORTED_VERSION: u32 = 7100;
const UPPER_SUPPORTED_VERSION: u32 = 7400;

#[derive(Debug)]
pub enum DocumentParseError {
    ParserError(ParserError),
    UnsupportedVersion(u32, Option<String>),
    RequiredElementNotFound(String),
    ElementParseError(ElementParseError),
}

#[derive(Debug, Default)]
pub struct ImportSettings {
    pub strict: bool,
}

#[derive(Default)]
pub struct Document {
    fbx_version: u32,
    creator: String,
    creation_date: [u32; 7],
}

trait HeaderReader {
    fn read_header(
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
        elements.read_header(&mut document, settings)?;
        Ok(document)
    }
}

impl HeaderReader for ElementAmphitheatre {
    fn read_header(
        &self,
        document: &mut Document,
        settings: ImportSettings,
    ) -> Result<(), DocumentParseError> {
        let header_extension = self.get_handle_by_key("FBXHeaderExtension");
        if header_extension.is_none() {
            return Err(DocumentParseError::RequiredElementNotFound(
                "FBXHeaderExtension".to_string(),
            ));
        }
        let header_extension = header_extension.unwrap();

        // Handle FBXVersion element
        let version_element = header_extension.first_child_by_key("FBXVersion").ok_or(
            DocumentParseError::RequiredElementNotFound("FBXVersion".to_string()),
        )?;
        document.fbx_version = version_element
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;

        // Check if the version is supported
        if document.fbx_version < LOWEST_SUPPORTED_VERSION {
            return Err(DocumentParseError::UnsupportedVersion(
                document.fbx_version,
                None,
            ));
        }
        if document.fbx_version > UPPER_SUPPORTED_VERSION && settings.strict {
            return Err(DocumentParseError::UnsupportedVersion(
                document.fbx_version,
                Some("Turn off strict mode to import this version.".to_string()),
            ));
        }

        // Handle Creator element
        document.creator = header_extension
            .first_child_by_key("Creator")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Creator".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;

        // Handle CreationTimeStamp element    
        let creation_date_element = header_extension
            .first_child_by_key("CreationTimeStamp")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "CreationTimeStamp".to_string(),
            ))?;
        let year = creation_date_element
            .first_child_by_key("Year")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Year".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        let month = creation_date_element
            .first_child_by_key("Month")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Month".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        let day = creation_date_element
            .first_child_by_key("Day")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Day".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        let hour = creation_date_element
            .first_child_by_key("Hour")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Hour".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        let minute = creation_date_element
            .first_child_by_key("Minute")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Minute".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        let second = creation_date_element
            .first_child_by_key("Second")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Second".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        let millisecond = creation_date_element
            .first_child_by_key("Millisecond")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Millisecond".to_string(),
            ))?
            .try_into()
            .map_err(DocumentParseError::ElementParseError)?;
        document.creation_date = [year, month, day, hour, minute, second, millisecond];
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use fbxscii::{Parser, Tokenizer};
    use std::io::BufReader;

    use crate::document::{Document, ImportSettings};

    #[test]
    fn test_document_parse() {
        let test_document = r#"
FBXHeaderExtension:  {
	FBXHeaderVersion: 1003
	FBXVersion: 7300
	CreationTimeStamp:  {
		Version: 1000
		Year: 2012
		Month: 6
		Day: 28
		Hour: 16
		Minute: 32
		Second: 53
		Millisecond: 433
	}
	Creator: "FBX SDK/FBX Plugins version 2013.1"
}"#;
        let tokenizer = Tokenizer::new(BufReader::new(test_document.as_bytes()));
        let parser = Parser::new(tokenizer);
        let document = Document::from_parser(parser, ImportSettings::default()).unwrap();
        assert_eq!(document.fbx_version, 7300);
        assert_eq!(document.creator, "FBX SDK/FBX Plugins version 2013.1");
        assert_eq!(document.creation_date, [2012, 6, 28, 16, 32, 53, 433]);
    }
}
