//! Binary FBX (fbxcel **v7400** tree) → same [`crate::Document`] fields as ASCII.
//!
//! Node walks mirror the ASCII loader: header, definitions, `Objects`, `Connections`. Attribute
//! values are coerced to [`crate::document::Property`] where possible; connection rows become
//! `OO` / `OP` / `PP` entries. See [`crate::document`] for semantics.

use fbxcel::{
    low::{FbxVersion, v7400::AttributeValue},
    tree::{any::AnyTree, v7400::NodeHandle},
};
use fbxscii::{Element, ElementAmphitheatre};

use crate::{
    document::{
        Document, DocumentLoader, DocumentParseError, ImportSettings, LazyObject,
        ObjectPropertyConnection, Property, PropertyDetails, Template,
    },
    loader::{LOWEST_SUPPORTED_VERSION, UPPER_SUPPORTED_VERSION},
};

/// Converts [`FbxVersion`] major/minor into the same numeric `FBXVersion` style used in ASCII headers (e.g. 7.4 → 7400).
fn fbx_version_to_u32(version: FbxVersion) -> u32 {
    (version.major() * 1000) + (version.minor() * 100)
}

/// Lossy conversion of a single FBX binary attribute to a UTF-8 string when the value is scalar or textual.
trait IntoAttributeString {
    /// Returns a string representation, or `None` for array and opaque binary payloads.
    fn into_attribute_string(&self) -> Option<String>;
}

impl IntoAttributeString for AttributeValue {
    fn into_attribute_string(&self) -> Option<String> {
        match self {
            AttributeValue::String(value) => Some(value.to_owned()),
            AttributeValue::I16(value) => Some(value.to_string()),
            AttributeValue::I32(value) => Some(value.to_string()),
            AttributeValue::I64(value) => Some(value.to_string()),
            AttributeValue::F32(value) => Some(value.to_string()),
            AttributeValue::F64(value) => Some(value.to_string()),
            AttributeValue::Bool(value) => Some(value.to_string()),
            AttributeValue::Binary(_)
            | AttributeValue::ArrBool(_)
            | AttributeValue::ArrI32(_)
            | AttributeValue::ArrI64(_)
            | AttributeValue::ArrF32(_)
            | AttributeValue::ArrF64(_) => None,
        }
    }
}

/// Reads a single attribute as `u32` when it is stored as a numeric scalar in the binary tree.
trait IntoAttributeU32 {
    /// Parses integer-like scalars; returns `None` for strings, arrays, and non-integer types.
    fn into_attribute_u32(&self) -> Option<u32>;
}

impl IntoAttributeU32 for AttributeValue {
    fn into_attribute_u32(&self) -> Option<u32> {
        match self {
            AttributeValue::I16(value) => Some(*value as u32),
            AttributeValue::I32(value) => Some(*value as u32),
            AttributeValue::I64(value) => Some(*value as u32),
            AttributeValue::F32(value) => Some(*value as u32),
            AttributeValue::F64(value) => Some(*value as u32),
            AttributeValue::Bool(_)
            | AttributeValue::Binary(_)
            | AttributeValue::ArrBool(_)
            | AttributeValue::ArrI32(_)
            | AttributeValue::ArrI64(_)
            | AttributeValue::ArrF32(_)
            | AttributeValue::ArrF64(_)
            | AttributeValue::String(_) => None,
        }
    }
}

/// Reads a single attribute as `u64` when it is stored as a numeric scalar in the binary tree.
trait IntoAttributeU64 {
    /// Parses integer-like scalars; returns `None` for strings, arrays, and non-integer types.
    fn into_attribute_u64(&self) -> Option<u64>;
}

impl IntoAttributeU64 for AttributeValue {
    fn into_attribute_u64(&self) -> Option<u64> {
        match self {
            AttributeValue::I16(value) => Some(*value as u64),
            AttributeValue::I32(value) => Some(*value as u64),
            AttributeValue::I64(value) => Some(*value as u64),
            AttributeValue::F32(value) => Some(*value as u64),
            AttributeValue::F64(value) => Some(*value as u64),
            AttributeValue::Bool(_)
            | AttributeValue::Binary(_)
            | AttributeValue::ArrBool(_)
            | AttributeValue::ArrI32(_)
            | AttributeValue::ArrI64(_)
            | AttributeValue::ArrF32(_)
            | AttributeValue::ArrF64(_)
            | AttributeValue::String(_) => None,
        }
    }
}

/// Fills `document` header fields from `FBXHeaderExtension` under the fbxcel root (version, creator, timestamp).
fn read_header_from_tree(
    root: NodeHandle<'_>,
    document: &mut Document,
    settings: ImportSettings,
) -> Result<(), DocumentParseError> {
    let header_extension = root.first_child_by_name("FBXHeaderExtension").ok_or(
        DocumentParseError::RequiredElementNotFound("FBXHeaderExtension".to_string()),
    )?;

    // Get FBXVersion from loaded tree.
    let missing = || DocumentParseError::RequiredElementNotFound("FBXVersion".into());
    document.fbx_version = header_extension
        .first_child_by_name("FBXVersion")
        .ok_or_else(missing)?
        .attributes()
        .get(0)
        .and_then(|v| v.into_attribute_u32())
        .ok_or_else(missing)?;

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

    // Get Creator from loaded tree.
    let missing = || DocumentParseError::RequiredElementNotFound("Creator".into());
    document.creator = header_extension
        .first_child_by_name("Creator")
        .ok_or_else(missing)?
        .attributes()
        .get(0)
        .and_then(|value| value.into_attribute_string())
        .ok_or_else(missing)?
        .to_owned();

    // Get CreationTimeStamp from loaded tree.
    let missing = || DocumentParseError::RequiredElementNotFound("CreationTimeStamp".into());
    let missing_child = |child_name: &str| {
        DocumentParseError::RequiredElementNotFound(format!("{} in CreationTimeStamp", child_name))
    };
    let creation_date_element = header_extension
        .first_child_by_name("CreationTimeStamp")
        .ok_or_else(missing)?;
    let year = creation_date_element
        .first_child_by_name("Year")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Year"))?;
    let month = creation_date_element
        .first_child_by_name("Month")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Month"))?;
    let day = creation_date_element
        .first_child_by_name("Day")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Day"))?;
    let hour = creation_date_element
        .first_child_by_name("Hour")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Hour"))?;
    let minute = creation_date_element
        .first_child_by_name("Minute")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Minute"))?;
    let second = creation_date_element
        .first_child_by_name("Second")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Second"))?;
    let millisecond = creation_date_element
        .first_child_by_name("Millisecond")
        .and_then(|node| {
            node.attributes()
                .get(0)
                .and_then(|value| value.into_attribute_u32())
        })
        .ok_or_else(|| missing_child("Millisecond"))?;

    document.creation_date = [year, month, day, hour, minute, second, millisecond];
    Ok(())
}

/// Parses one `Properties70` child node (typically `P`) from binary attributes into a [`PropertyDetails`].
fn read_property_details_from_tree(node: NodeHandle<'_>) -> Option<PropertyDetails> {
    let attributes = node.attributes();
    if attributes.len() < 2 {
        return None;
    }

    let property_name = attributes
        .get(0)
        .and_then(|value| value.into_attribute_string())?;
    let property_type = attributes
        .get(1)
        .and_then(|value| value.into_attribute_string())?;
    let property = match property_type.as_str() {
        "KString" => Property::String(
            attributes
                .get(4)
                .and_then(|value| value.into_attribute_string())?,
        ),
        "bool" | "Bool" => {
            let value = attributes
                .get(4)
                .and_then(|value| value.into_attribute_u32())
                .unwrap_or_default();
            Property::Bool(value != 0)
        }
        "int" | "Int" | "enum" | "Enum" | "Integer" => {
            let value = attributes
                .get(4)
                .and_then(|value| value.into_attribute_u32())
                .and_then(|v| i32::try_from(v).ok())
                .unwrap_or_default();
            Property::Int(value)
        }
        "ULongLong" => Property::ULongLong(
            attributes
                .get(4)
                .and_then(|value| value.into_attribute_u64())
                .unwrap_or_default(),
        ),
        "KTime" => {
            let value = attributes
                .get(4)
                .and_then(|value| value.into_attribute_u64())
                .and_then(|v| i64::try_from(v).ok())
                .unwrap_or_default();
            Property::ILongLong(value)
        }
        "double" | "Number" | "float" | "Float" | "FieldOfView" | "UnitScaleFactor" => {
            let value = attributes
                .get(4)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            Property::Float(value)
        }
        "Vector3D" | "Vector" | "Color" | "ColorRGB" | "Lcl Translation" | "Lcl Rotation"
        | "Lcl Scaling" => {
            let x = attributes
                .get(4)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            let y = attributes
                .get(5)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            let z = attributes
                .get(6)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            Property::Vec3([x, y, z])
        }
        "ColorAndAlpha" => {
            let r = attributes
                .get(4)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            let g = attributes
                .get(5)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            let b = attributes
                .get(6)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            let a = attributes
                .get(7)
                .and_then(|entry| entry.get_f64().or_else(|| entry.get_f32().map(f64::from)))
                .unwrap_or_default() as f32;
            Property::Vec4([r, g, b, a])
        }
        _ => return None,
    };

    Some(PropertyDetails {
        name: property_name,
        property,
    })
}

/// Loads `Definitions` → templates and the per-`ObjectType` default template key map from the fbxcel tree.
fn read_definitions_from_tree(
    root: NodeHandle<'_>,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let definitions = root.first_child_by_name("Definitions").ok_or(
        DocumentParseError::RequiredElementNotFound("Definitions".to_string()),
    )?;

    for object_type in definitions.children_by_name("ObjectType") {
        let Some(object_name) = object_type
            .attributes()
            .get(0)
            .and_then(|value| value.into_attribute_string())
        else {
            continue;
        };
        for property_template in object_type.children_by_name("PropertyTemplate") {
            let Some(property_name) = property_template
                .attributes()
                .get(0)
                .and_then(|value| value.into_attribute_string())
            else {
                continue;
            };

            if let Some(properties70) = property_template.first_child_by_name("Properties70") {
                let template_name = format!("{object_name}.{property_name}");
                document
                    .default_template_by_object_type
                    .entry(object_name.clone())
                    .or_insert_with(|| template_name.clone());
                let template = document
                    .templates
                    .entry(template_name)
                    .or_insert(Template::default());
                for property in properties70.children() {
                    if let Some(property_details) = read_property_details_from_tree(property) {
                        template.insert(property_details.name, property_details.property);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Loads `GlobalSettings.Properties70` into `document.global_settings`.
fn read_global_settings_from_tree(
    root: NodeHandle<'_>,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let global_settings = root.first_child_by_name("GlobalSettings").ok_or(
        DocumentParseError::RequiredElementNotFound("GlobalSettings".to_string()),
    )?;
    let properties70 = global_settings.first_child_by_name("Properties70").ok_or(
        DocumentParseError::RequiredElementNotFound("GlobalSettings.Properties70".to_string()),
    )?;
    for property in properties70.children() {
        if let Some(property_details) = read_property_details_from_tree(property) {
            document
                .global_settings
                .insert(property_details.name, property_details.property);
        }
    }
    Ok(())
}

/// Serializes one binary [`AttributeValue`] to a string token for mirroring into [`fbxscii::Element::tokens`].
fn attribute_value_to_ascii_token(value: &AttributeValue) -> String {
    match value {
        AttributeValue::Bool(b) => {
            if *b {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        AttributeValue::I16(v) => v.to_string(),
        AttributeValue::I32(v) => v.to_string(),
        AttributeValue::I64(v) => v.to_string(),
        AttributeValue::F32(v) => format!("{v}"),
        AttributeValue::F64(v) => format!("{v}"),
        AttributeValue::String(s) => s.clone(),
        AttributeValue::Binary(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        AttributeValue::ArrBool(values) => values
            .iter()
            .map(|b| if *b { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join(","),
        AttributeValue::ArrI32(values) => values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
        AttributeValue::ArrI64(values) => values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
        AttributeValue::ArrF32(values) => values
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(","),
        AttributeValue::ArrF64(values) => values
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(","),
    }
}

/// Returns the arena index of the synthetic `Objects` root element, creating it if absent.
fn ensure_objects_amphitheatre_root(arena: &mut ElementAmphitheatre) -> usize {
    if let Some(handle) = arena.get_handle_by_key("Objects") {
        return handle.index();
    }
    arena.insert(Element::new("Objects".into()))
}

/// Inserts `element` into `arena`, links it as a child of `parent_idx`, and returns the new element index.
fn append_element_child(
    arena: &mut ElementAmphitheatre,
    parent_idx: usize,
    mut element: Element,
) -> usize {
    element.parent_index = Some(parent_idx);
    let child_idx = arena.insert(element);
    if let Some(parent) = arena.get_mut(parent_idx) {
        parent.children.push(child_idx);
    }
    child_idx
}

/// Recursively copies a fbxcel subtree under `parent_idx` as `fbxscii::Element` nodes (key + tokenized attributes).
fn mirror_fbxcel_subtree(arena: &mut ElementAmphitheatre, parent_idx: usize, node: NodeHandle<'_>) {
    let key = node.name().to_string();
    let tokens: Vec<String> = node
        .attributes()
        .iter()
        .map(attribute_value_to_ascii_token)
        .collect();
    let mut element = Element::new(key);
    element.tokens = tokens;
    let idx = append_element_child(arena, parent_idx, element);
    for child in node.children() {
        mirror_fbxcel_subtree(arena, idx, child);
    }
}

/// Loads `Objects` into `document.objects` and mirrors each object subtree into `object_element_amphitheatre` under `Objects`.
fn read_objects_from_tree(
    root: NodeHandle<'_>,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let objects =
        root.first_child_by_name("Objects")
            .ok_or(DocumentParseError::RequiredElementNotFound(
                "Objects".to_string(),
            ))?;

    let objects_parent_idx = {
        let arena = &mut document.object_element_amphitheatre;
        ensure_objects_amphitheatre_root(arena)
    };

    for object in objects.children() {
        let attributes = object.attributes();
        if attributes.len() < 3 {
            continue;
        }
        let Some(object_index) = attributes
            .get(0)
            .and_then(|value| value.into_attribute_u64())
        else {
            continue;
        };
        let Some(name) = attributes
            .get(1)
            .and_then(|value| value.into_attribute_string())
        else {
            continue;
        };
        let Some(class_name) = attributes
            .get(2)
            .and_then(|value| value.into_attribute_string())
        else {
            continue;
        };

        let type_name = object.name().to_string();
        let element_index = {
            let arena = &mut document.object_element_amphitheatre;
            let mut obj_el = Element::new(type_name.clone());
            obj_el.tokens = vec![object_index.to_string(), name.clone(), class_name.clone()];
            let idx = append_element_child(arena, objects_parent_idx, obj_el);
            for child in object.children() {
                mirror_fbxcel_subtree(arena, idx, child);
            }
            idx
        };

        document.objects.insert(
            object_index,
            LazyObject {
                name,
                type_name,
                class_name,
                element_index,
            },
        );
    }
    Ok(())
}

/// Parses `Connections` (`OO` / `OP` / `PP`) from binary node attributes into the document connection maps.
fn read_connections_from_tree(
    root: NodeHandle<'_>,
    document: &mut Document,
) -> Result<(), DocumentParseError> {
    let connections = root.first_child_by_name("Connections").ok_or(
        DocumentParseError::RequiredElementNotFound("Connections".to_string()),
    )?;

    for connection in connections.children() {
        let attributes = connection.attributes();
        let Some(connection_type) = attributes
            .get(0)
            .and_then(|value| value.into_attribute_string())
        else {
            continue;
        };
        match connection_type.as_str() {
            "OO" => {
                let Some(src) = attributes
                    .get(1)
                    .and_then(|value| value.into_attribute_u64())
                else {
                    continue;
                };
                let Some(dest) = attributes
                    .get(2)
                    .and_then(|value| value.into_attribute_u64())
                else {
                    continue;
                };
                document
                    .object_connections
                    .entry(src)
                    .or_default()
                    .push(dest);
            }
            "OP" => {
                let Some(src) = attributes
                    .get(1)
                    .and_then(|value| value.into_attribute_u64())
                else {
                    continue;
                };
                let Some(dest) = attributes
                    .get(2)
                    .and_then(|value| value.into_attribute_u64())
                else {
                    continue;
                };
                let Some(property) = attributes
                    .get(3)
                    .and_then(|value| value.into_attribute_string())
                else {
                    continue;
                };
                document
                    .object_property_connections
                    .entry(src)
                    .or_default()
                    .push(ObjectPropertyConnection { dest, property });
            }
            "PP" => {
                let Some(src) = attributes
                    .get(1)
                    .and_then(|value| value.into_attribute_u64())
                else {
                    continue;
                };
                let Some(src_property) = attributes
                    .get(2)
                    .and_then(|value| value.into_attribute_string())
                else {
                    continue;
                };
                let Some(dest) = attributes
                    .get(3)
                    .and_then(|value| value.into_attribute_u64())
                else {
                    continue;
                };
                let Some(dest_property) = attributes
                    .get(4)
                    .and_then(|value| value.into_attribute_string())
                else {
                    continue;
                };
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
            _ => {}
        }
    }

    Ok(())
}

impl DocumentLoader for AnyTree {
    /// Dispatches on [`AnyTree`] variant and populates [`Document`] from the v7400 tree (header through connections).
    fn load_into_document(
        self,
        document: &mut Document,
        settings: ImportSettings,
    ) -> Result<(), DocumentParseError> {
        match self {
            AnyTree::V7400(fbx_version, tree, _footer) => {
                document.fbx_version = fbx_version_to_u32(fbx_version);
                let root = tree.root();
                read_header_from_tree(root, document, settings)?;
                read_definitions_from_tree(root, document)?;
                read_global_settings_from_tree(root, document)?;
                read_objects_from_tree(root, document)?;
                read_connections_from_tree(root, document)?;
                Ok(())
            }
            _ => Err(DocumentParseError::BinaryParserError(
                "Unsupported fbxcel tree version".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use fbxcel::tree_v7400;

    use crate::document::{Document, ImportSettings, ObjectPropertyConnection, Property};
    use crate::{ClassifiedFbxObject, FbxTryFromReason, OwnedObject};

    use super::{
        read_connections_from_tree, read_definitions_from_tree, read_global_settings_from_tree,
        read_header_from_tree, read_objects_from_tree,
    };

    /// End-to-end smoke test: minimal v7400 tree → document fields, amphitheatre objects, and connection maps.
    #[test]
    fn test_tree_mapping_spike() {
        let tree = tree_v7400! {
            FBXHeaderExtension: {
                FBXVersion: [7400i32] {}
                Creator: ["fbxcel-tree-test"] {}
                CreationTimeStamp: {
                    Year: [2026i32] {}
                    Month: [4i32] {}
                    Day: [16i32] {}
                    Hour: [9i32] {}
                    Minute: [30i32] {}
                    Second: [45i32] {}
                    Millisecond: [12i32] {}
                }
            }
            Definitions: {
                ObjectType: ["Geometry"] {
                    PropertyTemplate: ["FbxMesh"] {
                        Properties70: {
                            P: ["UnitScaleFactor", "double", "", "A", 100.0f64] {}
                        }
                    }
                }
            }
            GlobalSettings: {
                Properties70: {
                    P: ["UpAxis", "int", "", "A", 1i32] {}
                }
            }
            Objects: {
                Geometry: [101i64, "Geometry::Cube", "Mesh"] {}
            }
            Connections: {
                C: ["OO", 101i64, 0i64] {}
                C: ["OP", 101i64, 0i64, "DiffuseColor"] {}
                C: ["PP", 101i64, "SourceProp", 0i64, "DestProp"] {}
            }
        };

        let root = tree.root();
        let mut document = Document::default();
        read_header_from_tree(root, &mut document, ImportSettings::default()).unwrap();
        read_definitions_from_tree(root, &mut document).unwrap();
        read_global_settings_from_tree(root, &mut document).unwrap();
        read_objects_from_tree(root, &mut document).unwrap();
        read_connections_from_tree(root, &mut document).unwrap();

        assert_eq!(document.version(), 7400);
        assert_eq!(document.creator(), "fbxcel-tree-test");
        assert_eq!(document.creation_date(), &[2026, 4, 16, 9, 30, 45, 12]);
        assert_eq!(
            document
                .templates
                .get("Geometry.FbxMesh")
                .and_then(|template| template.get("UnitScaleFactor")),
            Some(&Property::Float(100.0))
        );
        assert_eq!(
            document.global_settings.get("UpAxis"),
            Some(&Property::Int(1))
        );
        assert!(document.objects.contains_key(&101));
        assert_eq!(document.object_connections.get(&101), Some(&vec![0]));
        assert_eq!(
            document.object_to_source_properties.get(&101),
            Some(&vec!["SourceProp".to_string()])
        );

        let object = document.object_by_index(101).expect("object 101");
        assert_eq!(object.connected_object_ids(), &[0]);
        assert_eq!(
            object.object_property_targets(),
            &[ObjectPropertyConnection {
                dest: 0,
                property: "DiffuseColor".to_string(),
            }]
        );
        assert_eq!(
            object.pp_source_property_names(),
            &["SourceProp".to_string()]
        );
        assert_eq!(
            object.pp_targets((101, "SourceProp")),
            Some(
                &[ObjectPropertyConnection {
                    dest: 0,
                    property: "DestProp".to_string(),
                }][..],
            )
        );

        let owned: OwnedObject = document.object_by_index(101).unwrap().into();
        assert_eq!(owned.object_index, 101);
        assert_eq!(owned.connected_object_ids, vec![0]);
        assert_eq!(
            owned.object_property_targets,
            vec![ObjectPropertyConnection {
                dest: 0,
                property: "DiffuseColor".to_string(),
            }]
        );
        assert_eq!(
            owned.pp_property_targets.get("SourceProp"),
            Some(&ObjectPropertyConnection {
                dest: 0,
                property: "DestProp".to_string(),
            })
        );

        // Minimal geometry node has no mesh vertex data (`Vertices` / `PolygonVertexIndex`).
        let err = ClassifiedFbxObject::try_from(owned).unwrap_err();
        assert!(matches!(
            err.reason,
            FbxTryFromReason::MissingAttribute { ref name } if name == "Vertices"
        ));
    }
}
