use fbxscii::{ElementAmphitheatre, ElementHandle};

use crate::document::{
    Document, DocumentLoader, DocumentParseError, ImportSettings, LazyObject, ObjectPropertyConnection,
    Property, PropertyDetails, PropertyParseError, Template,
};

pub const LOWEST_SUPPORTED_VERSION: u32 = 7100;
pub const UPPER_SUPPORTED_VERSION: u32 = 7400;

fn read_header(
    amphitheatre: &ElementAmphitheatre,
    document: &mut Document,
    settings: ImportSettings,
) -> Result<(), DocumentParseError> {
    let header_extension = amphitheatre.get_handle_by_key("FBXHeaderExtension");
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

fn read_definitions(
    amphitheatre: &ElementAmphitheatre,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let definition_handle_opt = amphitheatre.get_handle_by_key("Definitions");
    if definition_handle_opt.is_none() {
        return Ok(());
    }
    let definition_handle = definition_handle_opt.unwrap();
    let object_type_handles = definition_handle.children_by_key("ObjectType");
    for object_type_handle in object_type_handles {
        if !object_type_handle.has_children() {
            continue;
        }

        let object_tokens = object_type_handle.tokens();
        if object_tokens.is_empty() {
            continue;
        }
        let object_name = &object_tokens[0];
        let property_template_handles = object_type_handle.children_by_key("PropertyTemplate");
        for property_template_handle in property_template_handles {
            if !property_template_handle.has_children() {
                continue;
            }
            let property_tokens = property_template_handle.tokens();
            if property_tokens.is_empty() {
                continue;
            }
            let property_name = &property_tokens[0];
            let property_table_handle_opt =
                property_template_handle.first_child_by_key("Properties70");
            if let Some(property_table_handle) = property_table_handle_opt {
                let template_name = format!("{}.{}", object_name, property_name);
                document
                    .default_template_by_object_type
                    .entry(object_name.to_string())
                    .or_insert_with(|| template_name.clone());
                let template = document
                    .templates
                    .entry(template_name)
                    .or_insert(Template::default());
                for property_detail in property_table_handle.children() {
                    let r: Result<PropertyDetails, _> = property_detail.try_into();
                    if let Ok(property_details) = r {
                        template.insert(property_details.name, property_details.property);
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_global_settings(
    amphitheatre: &ElementAmphitheatre,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let global_settings_handle_opt = amphitheatre.get_handle_by_key("GlobalSettings");
    if global_settings_handle_opt.is_none() {
        return Ok(());
    }
    let global_settings_handle = global_settings_handle_opt.unwrap();
    let property_table_handle_opt = global_settings_handle.first_child_by_key("Properties70");
    if let Some(property_table_handle) = property_table_handle_opt {
        for property_detail in property_table_handle.children() {
            let property_details: PropertyDetails = property_detail
                .try_into()
                .map_err(DocumentParseError::PropertyParseError)?;
            document
                .global_settings
                .insert(property_details.name, property_details.property);
        }
    }
    Ok(())
}

fn read_connections(
    amphitheatre: &ElementAmphitheatre,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let connections_handle_opt = amphitheatre.get_handle_by_key("Connections");
    if connections_handle_opt.is_none() {
        return Ok(());
    }
    let connections_handle = connections_handle_opt.unwrap();
    // Read Object Connections
    // Connections of the following format:
    // C: "OO",40530896,0
    // First token is always "C"
    // Second token is the connection type ("OO" or "OP" or "PP") (Object to Object, Object to Property, Property to Property)
    // If the connection is OO, it should have 2 more tokens:
    // the source object index, and the destination object index.
    // If the connection is OP, it should have 3 more tokens:
    // the source object index, the destination object index, and the property name.
    // If the connection is PP, it should have 4 more tokens:
    // the source object index, the source property name, the destination object index, and the destination property name.
    for connection_handle in connections_handle.children() {
        let connection_tokens = connection_handle.tokens();
        if connection_tokens.len() < 2 {
            continue;
        }
        let connection_type = &connection_tokens[1];
        match connection_type.as_str() {
            "OO" => {
                if connection_tokens.len() != 4 {
                    continue;
                }
                let src = connection_tokens[2].parse::<u64>().unwrap_or(0);
                let dest = connection_tokens[3].parse::<u64>().unwrap_or(0);
                document
                    .object_connections
                    .entry(src)
                    .or_insert(Vec::new())
                    .push(dest);
            }
            "OP" => {
                if connection_tokens.len() != 5 {
                    continue;
                }
                let src = connection_tokens[2].parse::<u64>().unwrap_or(0);
                let dest = connection_tokens[3].parse::<u64>().unwrap_or(0);
                let property = connection_tokens[4].to_string();
                document
                    .object_property_connections
                    .entry(src)
                    .or_insert(Vec::new())
                    .push(ObjectPropertyConnection { dest, property });
            }
            "PP" => {
                if connection_tokens.len() != 6 {
                    continue;
                }
                let src = connection_tokens[2].parse::<u64>().unwrap_or(0);
                let src_property = connection_tokens[3].to_string();
                let dest = connection_tokens[4].parse::<u64>().unwrap_or(0);
                let dest_property = connection_tokens[5].to_string();
                document
                    .object_to_source_properties
                    .entry(src)
                    .or_insert(Vec::new())
                    .push(src_property.clone());
                document
                    .property_connections
                    .entry(ObjectPropertyConnection {
                        dest: src,
                        property: src_property,
                    })
                    .or_insert(Vec::new())
                    .push(ObjectPropertyConnection {
                        dest,
                        property: dest_property,
                    });
            }
            _ => continue,
        }
    }
    Ok(())
}
fn read_objects(
    amphitheatre: &ElementAmphitheatre,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let objects_handle_opt = amphitheatre.get_handle_by_key("Objects");
    if objects_handle_opt.is_none() {
        return Ok(());
    }
    let objects_handle = objects_handle_opt.unwrap();
    for object_handle in objects_handle.children() {
        let object_tokens = object_handle.tokens();
        if object_tokens.len() != 3 {
            continue;
        }
        let object_index = object_tokens[0].parse::<u64>();
        if object_index.is_err() {
            continue;
        }
        let object_index = object_index.unwrap();

        let object = LazyObject {
            name: object_tokens[1].to_string(),
            type_name: object_handle.key().to_string(),
            class_name: object_tokens[2].to_string(),
            element_index: object_handle.index(),
        };
        document.objects.insert(object_index, object);
    }
    Ok(())
}

impl<'a> TryFrom<ElementHandle<'a>> for PropertyDetails {
    type Error = PropertyParseError;

    fn try_from(handle: ElementHandle<'a>) -> Result<Self, PropertyParseError> {
        let tokens = handle.tokens();
        if tokens.len() < 2 {
            return Err(PropertyParseError::InvalidTokenLength(tokens.len(), None));
        }
        let property_name = &tokens[0];
        let property_type = &tokens[1];
        let property = match property_type.as_str() {
            "KString" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                Property::String(tokens[4].to_string())
            }
            "bool" | "Bool" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let val = tokens[4].parse::<i32>().unwrap_or(0);
                Property::Bool(val != 0)
            }
            "int" | "Int" | "enum" | "Enum" | "Integer" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let val = tokens[4].parse::<i32>().unwrap_or(0);
                Property::Int(val)
            }
            "ULongLong" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let val = tokens[4].parse::<u64>().unwrap_or(0);
                Property::ULongLong(val)
            }
            "KTime" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let val = tokens[4].parse::<i64>().unwrap_or(0);
                Property::ILongLong(val)
            }
            "double" | "Number" | "float" | "Float" | "FieldOfView" | "UnitScaleFactor" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let val = tokens[4].parse::<f32>().unwrap_or(0.0);
                Property::Float(val)
            }
            "Vector3D" | "Vector" | "Color" | "ColorRGB" | "Lcl Translation" | "Lcl Rotation"
            | "Lcl Scaling" => {
                if tokens.len() != 7 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let x = tokens[4].parse::<f32>().unwrap_or(0.0);
                let y = tokens[5].parse::<f32>().unwrap_or(0.0);
                let z = tokens[6].parse::<f32>().unwrap_or(0.0);
                Property::Vec3([x, y, z])
            }
            "ColorAndAlpha" => {
                if tokens.len() != 8 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let r = tokens[4].parse::<f32>().unwrap_or(0.0);
                let g = tokens[5].parse::<f32>().unwrap_or(0.0);
                let b = tokens[6].parse::<f32>().unwrap_or(0.0);
                let a = tokens[7].parse::<f32>().unwrap_or(0.0);
                Property::Vec4([r, g, b, a])
            }
            value => return Err(PropertyParseError::MissingPropertyType(value.to_string())),
        };
        Ok(PropertyDetails {
            name: property_name.to_string(),
            property,
        })
    }
}


impl DocumentLoader for ElementAmphitheatre {
    fn load_into_document(
        self,
        document: &mut Document,
        settings: ImportSettings,
    ) -> Result<(), DocumentParseError> {
        read_header(&self, document, settings)?;
        read_definitions(&self, document)?;
        read_global_settings(&self, document)?;

        read_objects(&self, document)?;
        read_connections(&self, document)?;
        document.object_element_amphitheatre = self;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use fbxscii::{Parser, Tokenizer};
    use std::io::BufReader;

    use crate::document::{Document, DocumentParseError, ImportSettings};

    #[test]
    fn test_header_parse() {
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

    #[test]
    fn test_empty_document_parse() {
        let test_document = "";
        let tokenizer = Tokenizer::new(BufReader::new(test_document.as_bytes()));
        let parser = Parser::new(tokenizer);
        let document = Document::from_parser(parser, ImportSettings::default());
        assert!(document.is_err());
        assert_eq!(
            document.unwrap_err(),
            DocumentParseError::RequiredElementNotFound("FBXHeaderExtension".to_string())
        );
    }

}
