//! ASCII FBX → [`crate::Document`]: parse [`fbxscii`] tree, read header/definitions/objects/connections,
//! and populate [`crate::document::Document`].
//!
//! Object rows in `Objects` are keyed by id; each row’s subtree is extracted into
//! [`Document::object_element_amphitheatre`]. `C:` connection rows fill the `OO` / `OP` / `PP`
//! maps. Version bounds apply when [`ImportSettings::strict`] is relevant (see header handling).

use std::convert::TryFrom;

use fbxscii::{ElementAmphitheatre, ElementHandle};

use crate::document::{
    Document, DocumentLoader, DocumentParseError, ImportSettings, LazyObject,
    ObjectPropertyConnection, Property, PropertyDetails, PropertyParseError, Template,
};

/// Minimum `FBXVersion` supported by the ASCII loader (below this → [`DocumentParseError::UnsupportedVersion`]).
pub const LOWEST_SUPPORTED_VERSION: u32 = 7100;
/// `FBXVersion` above this is rejected when [`ImportSettings::strict`] is true.
pub const UPPER_SUPPORTED_VERSION: u32 = 7400;

/// Fills [`Document::fbx_version`], creator string, and creation timestamp from `FBXHeaderExtension`.
///
/// Enforces [`LOWEST_SUPPORTED_VERSION`] always, and [`UPPER_SUPPORTED_VERSION`] when `settings.strict`.
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

/// Loads `Definitions` → per-object-type default templates and full template maps from each
/// `PropertyTemplate`’s `Properties70` children.
///
/// Rows that fail [`PropertyDetails::try_from`] are skipped after a `log::debug!` (e.g. unsupported `"object"` types in shipped files).
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
                let template = document.templates.entry(template_name.clone()).or_default();
                for property_detail in property_table_handle.children() {
                    match PropertyDetails::try_from(property_detail) {
                        Ok(property_details) => {
                            template.insert(property_details.name, property_details.property);
                        }
                        Err(e) => {
                            log::debug!(
                                target: "fbx_dom",
                                "skipped Properties70 default-template entry (template={}, error={})",
                                template_name,
                                e
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Parses `GlobalSettings.Properties70` into [`Document::global_settings`] (all entries required to parse).
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

/// Reads `Connections` children: each row’s tokens are `[kind, …]` where `kind` is `OO`, `OP`, or `PP`.
///
/// - **`OO`** — `src, dest` → [`Document::object_connections`].
/// - **`OP`** — `src, dest, property` → [`Document::object_property_connections`].
/// - **`PP`** — `src, src_property, dest, dest_property` → [`Document::property_connections`] and
///   [`Document::object_to_source_properties`].
///
/// Malformed rows (wrong arity or non-numeric ids) are skipped.
fn read_connections(
    amphitheatre: &ElementAmphitheatre,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let connections_handle_opt = amphitheatre.get_handle_by_key("Connections");
    if connections_handle_opt.is_none() {
        return Ok(());
    }
    let connections_handle = connections_handle_opt.unwrap();
    for connection_handle in connections_handle.children() {
        let connection_tokens = connection_handle.tokens();
        if connection_tokens.len() < 2 {
            continue;
        }
        let connection_type = &connection_tokens[0];
        match connection_type.as_str() {
            "OO" => {
                if connection_tokens.len() != 3 {
                    continue;
                }
                let Ok(src) = connection_tokens[1].parse::<u64>() else {
                    continue;
                };
                let Ok(dest) = connection_tokens[2].parse::<u64>() else {
                    continue;
                };
                document
                    .object_connections
                    .entry(src)
                    .or_default()
                    .push(dest);
            }
            "OP" => {
                if connection_tokens.len() != 4 {
                    continue;
                }
                let Ok(src) = connection_tokens[1].parse::<u64>() else {
                    continue;
                };
                let Ok(dest) = connection_tokens[2].parse::<u64>() else {
                    continue;
                };
                let property = connection_tokens[3].to_string();
                document
                    .object_property_connections
                    .entry(src)
                    .or_default()
                    .push(ObjectPropertyConnection { dest, property });
            }
            "PP" => {
                if connection_tokens.len() != 5 {
                    continue;
                }
                let Ok(src) = connection_tokens[1].parse::<u64>() else {
                    continue;
                };
                let src_property = connection_tokens[2].to_string();
                let Ok(dest) = connection_tokens[3].parse::<u64>() else {
                    continue;
                };
                let dest_property = connection_tokens[4].to_string();
                document
                    .object_to_source_properties
                    .entry(src)
                    .or_default()
                    .push(src_property.clone());
                document
                    .property_connections
                    .entry(ObjectPropertyConnection {
                        dest: src,
                        property: src_property,
                    })
                    .or_default()
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

/// Inserts each `Objects` child into [`Document::objects`] by numeric id; stores the element index for later subtree resolution.
///
/// Expects three tokens per row: `id`, `name`, `class_name`. The element **key** becomes [`LazyObject::type_name`].
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

/// Parses one ASCII `P:` row (`Properties70` child): tokens `[name, type, …, value…]` into [`PropertyDetails`].
///
/// Supported `type` strings mirror common FBX SDK property kinds; unknown types return [`PropertyParseError::MissingPropertyType`].
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
                let Ok(val) = tokens[4].parse::<i32>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[4].to_string(),
                    ));
                };
                Property::Bool(val != 0)
            }
            "int" | "Int" | "enum" | "Enum" | "Integer" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let Ok(val) = tokens[4].parse::<i32>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[4].to_string(),
                    ));
                };
                Property::Int(val)
            }
            "ULongLong" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let Ok(val) = tokens[4].parse::<u64>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[4].to_string(),
                    ));
                };
                Property::ULongLong(val)
            }
            "KTime" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let Ok(val) = tokens[4].parse::<i64>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[4].to_string(),
                    ));
                };
                Property::ILongLong(val)
            }
            "double" | "Number" | "float" | "Float" | "FieldOfView" | "UnitScaleFactor" => {
                if tokens.len() != 5 {
                    return Err(PropertyParseError::InvalidTokenLength(
                        tokens.len(),
                        Some(property_type.to_string()),
                    ));
                }
                let Ok(val) = tokens[4].parse::<f32>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[4].to_string(),
                    ));
                };
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
                let Ok(x) = tokens[4].parse::<f32>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[4].to_string(),
                    ));
                };
                let Ok(y) = tokens[5].parse::<f32>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[5].to_string(),
                    ));
                };
                let Ok(z) = tokens[6].parse::<f32>() else {
                    return Err(PropertyParseError::TokenParseError(
                        property_type.to_string(),
                        tokens[6].to_string(),
                    ));
                };
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
    /// ASCII ingress: header → definitions → global settings → objects → connections, then stores `self` as the object arena.
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

    use std::convert::TryFrom;
    use std::io::BufReader;

    use fbxscii::{ElementAmphitheatre, ElementHandle, Parser, Tokenizer};

    use crate::document::{
        Document, DocumentParseError, ImportSettings, ObjectPropertyConnection, Property,
        PropertyDetails, PropertyParseError,
    };

    /// Minimal ASCII FBX (7.2) with required header + `GlobalSettings.Properties70` body + tail.
    fn minimal_ascii_fbx_7200(global_properties70_body: &str, tail: &str) -> String {
        format!(
            r#"; test
FBXHeaderExtension:  {{
	FBXHeaderVersion: 1003
	FBXVersion: 7200
	CreationTimeStamp:  {{
		Version: 1000
		Year: 2012
		Month: 6
		Day: 28
		Hour: 16
		Minute: 32
		Second: 53
		Millisecond: 433
	}}
	Creator: "test"
}}
GlobalSettings:  {{
	Properties70:  {{
{global_properties70_body}
	}}
}}
{tail}"#
        )
    }

    /// Parses `src` into an [`ElementAmphitheatre`] (tests only).
    fn load_arena(src: &str) -> ElementAmphitheatre {
        let tokenizer = Tokenizer::new(BufReader::new(src.as_bytes()));
        let parser = Parser::new(tokenizer);
        parser.load().expect("parse ASCII FBX")
    }

    /// First `P` element under `GlobalSettings` → `Properties70` (tests only).
    fn first_p_child_properties70(arena: &ElementAmphitheatre) -> ElementHandle<'_> {
        let gs = arena
            .get_handle_by_key("GlobalSettings")
            .expect("GlobalSettings");
        let p70 = gs.first_child_by_key("Properties70").expect("Properties70");
        p70.children().next().expect("at least one P row")
    }

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

    #[test]
    fn connections_pp_ascii_populates_property_maps() {
        let src = minimal_ascii_fbx_7200(
            r#"		P: "UpAxis", "int", "", "",1
"#,
            r#"Definitions:  {
	ObjectType: "Geometry" {
		PropertyTemplate: "FbxMesh" {
			Properties70:  {
				P: "UnitScaleFactor", "double", "", "",1
			}
		}
	}
}
Objects:  {
	Geometry: 101, "A", "Mesh" {
	}
	Geometry: 202, "B", "Mesh" {
	}
}
Connections:  {
	C: "PP",101,"SrcProp",202,"DstProp"
}
"#,
        );
        let tokenizer = Tokenizer::new(BufReader::new(src.as_bytes()));
        let parser = Parser::new(tokenizer);
        let document = Document::from_parser(parser, ImportSettings::default()).unwrap();

        let obj = document.object_by_index(101).expect("object 101");
        assert_eq!(obj.pp_source_property_names(), &["SrcProp".to_string()]);
        assert_eq!(
            obj.pp_targets((101, "SrcProp")),
            Some(
                &[ObjectPropertyConnection {
                    dest: 202,
                    property: "DstProp".to_string(),
                }][..],
            )
        );
    }

    #[test]
    fn connections_pp_ascii_appends_multiple_destinations_for_same_source() {
        let src = minimal_ascii_fbx_7200(
            r#"		P: "UpAxis", "int", "", "",1
"#,
            r#"Definitions:  {
	ObjectType: "Geometry" {
		PropertyTemplate: "FbxMesh" {
			Properties70:  {
				P: "UnitScaleFactor", "double", "", "",1
			}
		}
	}
}
Objects:  {
	Geometry: 101, "A", "Mesh" {
	}
	Geometry: 202, "B", "Mesh" {
	}
	Geometry: 303, "C", "Mesh" {
	}
}
Connections:  {
	C: "PP",101,"SrcProp",202,"DstA"
	C: "PP",101,"SrcProp",303,"DstB"
}
"#,
        );
        let tokenizer = Tokenizer::new(BufReader::new(src.as_bytes()));
        let parser = Parser::new(tokenizer);
        let document = Document::from_parser(parser, ImportSettings::default()).unwrap();

        let obj = document.object_by_index(101).expect("object 101");
        // One list entry per `PP` row (same name repeated when multiple rows share the source key).
        assert_eq!(
            obj.pp_source_property_names(),
            &["SrcProp".to_string(), "SrcProp".to_string()]
        );
        let targets = obj.pp_targets((101, "SrcProp")).expect("PP targets");
        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&ObjectPropertyConnection {
            dest: 202,
            property: "DstA".to_string(),
        }));
        assert!(targets.contains(&ObjectPropertyConnection {
            dest: 303,
            property: "DstB".to_string(),
        }));
    }

    #[test]
    fn property_details_try_from_int_kstring_and_vector3d() {
        let src = minimal_ascii_fbx_7200(
            r#"		P: "UpAxis", "int", "", "",2
		P: "LayerName", "KString", "", "", "hello"
		P: "Point", "Vector3D", "Vector", "",0,1,2
"#,
            "",
        );
        let arena = load_arena(&src);
        let gs = arena.get_handle_by_key("GlobalSettings").unwrap();
        let p70 = gs.first_child_by_key("Properties70").unwrap();
        let mut it = p70.children();
        let d0: PropertyDetails = PropertyDetails::try_from(it.next().unwrap()).unwrap();
        assert_eq!(d0.name, "UpAxis");
        assert_eq!(d0.property, Property::Int(2));
        let d1: PropertyDetails = PropertyDetails::try_from(it.next().unwrap()).unwrap();
        assert_eq!(d1.name, "LayerName");
        assert_eq!(d1.property, Property::String("hello".to_string()));
        let d2: PropertyDetails = PropertyDetails::try_from(it.next().unwrap()).unwrap();
        assert_eq!(d2.name, "Point");
        assert_eq!(d2.property, Property::Vec3([0.0, 1.0, 2.0]));
    }

    #[test]
    fn property_details_try_from_missing_property_type_object() {
        let src = minimal_ascii_fbx_7200(
            r#"		P: "SourceObject", "object", "", ""
"#,
            "",
        );
        let arena = load_arena(&src);
        let p = first_p_child_properties70(&arena);
        let err = PropertyDetails::try_from(p).unwrap_err();
        assert_eq!(
            err,
            PropertyParseError::MissingPropertyType("object".to_string())
        );
    }

    #[test]
    fn property_details_try_from_invalid_token_length() {
        let src = minimal_ascii_fbx_7200(
            r#"		P: "Short", "int"
"#,
            "",
        );
        let arena = load_arena(&src);
        let p = first_p_child_properties70(&arena);
        let err = PropertyDetails::try_from(p).unwrap_err();
        assert_eq!(
            err,
            PropertyParseError::InvalidTokenLength(2, Some("int".to_string()))
        );
    }

    #[test]
    fn property_details_try_from_token_parse_error() {
        let src = minimal_ascii_fbx_7200(
            r#"		P: "BadInt", "int", "", "", "not_an_int"
"#,
            "",
        );
        let arena = load_arena(&src);
        let p = first_p_child_properties70(&arena);
        let err = PropertyDetails::try_from(p).unwrap_err();
        assert_eq!(
            err,
            PropertyParseError::TokenParseError("int".to_string(), "not_an_int".to_string())
        );
    }
}
