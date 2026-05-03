//! Parsed FBX file as header metadata, an element arena for `Objects` rows, and connection graphs.
//!
//! ## Connections (ASCII `Connections` section)
//!
//! - **`OO`** — Object → object. Stored in [`Document::object_connections`]: `source_id → [dest_id, …]`.
//!   Typical: `Model` → `Geometry`, `Model` → `Material`.
//! - **`OP`** — Object → object property. [`Document::object_property_connections`]:
//!   `source_id → [{ dest, property }]`. Example: texture linked to material’s `DiffuseColor`.
//! - **`PP`** — Property → property. [`Document::property_connections`] maps a **source**
//!   `(object_id, property_name)` (packed as [`ObjectPropertyConnection`] with `dest` = source id)
//!   to one or more destination [`ObjectPropertyConnection`] rows.
//!
//! [`DocumentLoader`] is implemented by ASCII element lists and by the binary tree adapter so both
//! ingress paths share the same in-memory shape.

use fbxcel::tree::any::AnyTree;
use fbxscii::{ElementAmphitheatre, ElementParseError, Parser, ParserError};
use std::{
    collections::HashMap,
    io::{BufRead, Read, Seek},
};

use crate::{Object, global::GlobalSettings, object::Objects};

#[derive(Debug, PartialEq)]
pub enum DocumentParseError {
    ParserError(ParserError),
    BinaryParserError(String),
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
pub enum Property {
    String(String),
    Bool(bool),
    Int(i32),
    Float(f32),
    ULongLong(u64),
    ILongLong(i64),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
}

#[derive(Debug)]
pub struct PropertyDetails {
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
pub struct LazyObject {
    pub name: String,
    pub type_name: String,
    pub class_name: String,
    /// Index of this object’s root element in the document’s object element arena.
    ///
    /// Populated by both ASCII and binary loaders. [`crate::Object::element`] resolves this index
    /// while a [`Document`] is still live.
    pub element_index: usize,
}

/// Endpoint for `OP` / `PP` rows: **destination** object id and property name on that object.
///
/// For **`PP`** map **keys**, Assimp/FBX use the source side: [`ObjectPropertyConnection::dest`] is
/// the **source** object id and `property` the **source** property name; values are the actual
/// destination links. See [`crate::Object::pp_targets`].
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct ObjectPropertyConnection {
    pub dest: u64,
    pub property: String,
}

#[derive(Default, Debug, Clone)]
pub struct Document {
    /// The version of the FBX file
    pub(crate) fbx_version: u32,
    /// The creating program of the FBX file
    pub(crate) creator: String,
    /// The creation date of the FBX file
    pub(crate) creation_date: [u32; 7],
    /// The templates of the FBX file
    pub(crate) templates: HashMap<String, Template>,
    /// Maps `ObjectType` (e.g. `Geometry`) to the first full template key
    /// (`ObjectType.PropertyTemplate`, e.g. `Geometry.FbxMesh`) in file order.
    pub(crate) default_template_by_object_type: HashMap<String, String>,
    /// The global settings of the FBX file
    pub(crate) global_settings: Template,
    /// The element amphitheatre containing object information
    pub(crate) object_element_amphitheatre: ElementAmphitheatre,
    /// The objects of the FBX file
    pub(crate) objects: HashMap<u64, LazyObject>,
    /// The connections between objects
    pub(crate) object_connections: HashMap<u64, Vec<u64>>,
    /// The connections between object properties
    pub(crate) object_property_connections: HashMap<u64, Vec<ObjectPropertyConnection>>,
    /// The connections between properties
    pub(crate) property_connections:
        HashMap<ObjectPropertyConnection, Vec<ObjectPropertyConnection>>,
    /// For each source object id in a `PP` connection, the source-side property names on that object.
    pub(crate) object_to_source_properties: HashMap<u64, Vec<String>>,
}

impl Document {
    pub fn version(&self) -> u32 {
        self.fbx_version
    }

    pub fn creator(&self) -> &str {
        &self.creator
    }

    pub fn creation_date(&self) -> &[u32; 7] {
        &self.creation_date
    }

    pub fn global_settings(&self) -> GlobalSettings<'_> {
        GlobalSettings::new(self, &self.global_settings)
    }

    pub fn objects(&self) -> Objects<'_> {
        Objects {
            iter: self.objects.iter(),
            document: self,
        }
    }

    pub fn object_by_index(&self, index: u64) -> Option<Object<'_>> {
        let object = self.object_for_index(index)?;
        self.template_for_object(object)
            .map(|template| Object::new(self, template, object, index))
    }

    pub(crate) fn object_for_index(&self, index: u64) -> Option<&LazyObject> {
        self.objects.get(&index)
    }

    pub(crate) fn template_for_object(&self, object: &LazyObject) -> Option<&Template> {
        self.templates.get(&object.type_name).or_else(|| {
            self.default_template_by_object_type
                .get(&object.type_name)
                .and_then(|full_key| self.templates.get(full_key))
        })
    }

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

    /// Loads a binary FBX document through `fbxcel`'s tree API.
    ///
    /// This is the chosen primary binary ingress for now:
    /// - parse bytes into `AnyTree`
    /// - adapt tree nodes into the existing `Document` fields
    /// - drop the `AnyTree` after materialization so we do not persist
    ///   both a full tree and a long-lived binary DOM side-by-side.
    pub fn from_binary_reader<R>(
        reader: R,
        settings: ImportSettings,
    ) -> Result<Self, DocumentParseError>
    where
        R: Read + Seek,
    {
        let any_tree = AnyTree::from_seekable_reader(reader)
            .map_err(|error| DocumentParseError::BinaryParserError(error.to_string()))?;
        let mut document = Self::default();
        any_tree.load_into_document(&mut document, settings)?;
        Ok(document)
    }
}

pub trait DocumentLoader {
    fn load_into_document(
        self,
        document: &mut Document,
        settings: ImportSettings,
    ) -> Result<(), DocumentParseError>;
}
