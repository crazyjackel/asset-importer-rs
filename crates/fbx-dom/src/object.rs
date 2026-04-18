use fbxscii::ElementAttribute;

use crate::document::{
    Document, LazyObject, ObjectPropertyConnection, Property, PropertyDetails, Template,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ObjectError {
    MissingTemplate(String),
}

/// Object is a wrapper around a LazyObject that provides access to the object's properties and attributes.
#[derive(Debug)]
pub struct Object<'a> {
    document: &'a Document,
    template: &'a Template,
    object: &'a LazyObject,
    index: u64,
}

impl<'a> Object<'a> {
    pub fn name(&self) -> &str {
        &self.object.name   
    }

    pub fn type_name(&self) -> &str {
        &self.object.type_name
    }

    pub fn class_name(&self) -> &str {
        &self.object.class_name
    }

    pub fn object_index(&self) -> u64 {
        self.index
    }

    pub fn element(&self) -> Option<&'a fbxscii::Element> {
        self.document.object_element_amphitheatre.get(self.object.element_index)
    }

    /// Indices of objects connected from this one via `OO` rows (`C: "OO", self, dest`).
    pub fn connected_object_ids(&self) -> &[u64] {
        self.document
            .object_connections
            .get(&self.index)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// `OP` connections from this object: each entry is a destination object id and the
    /// destination-side property name (`C: "OP", self, dest, property`).
    pub fn object_property_targets(&self) -> &[ObjectPropertyConnection] {
        self.document
            .object_property_connections
            .get(&self.index)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Property names on this object that appear as the source side of `PP` connections.
    pub fn pp_source_property_names(&self) -> &[String] {
        self.document
            .object_to_source_properties
            .get(&self.index)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// `PP` targets for a source `(object_id, property_name)` key (`C: "PP", …`).
    ///
    /// The document’s `PP` map keys reuse [`ObjectPropertyConnection`]: for `PP` rows,
    /// `ObjectPropertyConnection::dest` holds the **source** object id and `property` the source
    /// property name.
    pub fn pp_targets(&self, key: (u64, &str)) -> Option<&[ObjectPropertyConnection]> {
        let (source_object, source_property) = key;
        self.document
            .property_connections
            .get(&ObjectPropertyConnection { dest: source_object, property: source_property.to_string() })
            .map(|v| v.as_slice())
    }
}

impl<'a> Object<'a> {
    pub fn new(document: &'a Document, template: &'a Template, object: &'a LazyObject, index: u64) -> Self {
        Self {
            document,
            template,
            object,
            index,
        }
    }

    pub fn properties(&self) -> HashMap<String, Property> {
        let object_index = self.object.element_index;
        let object_handle = self
            .document
            .object_element_amphitheatre
            .get_handle(object_index);
        if object_handle.is_none() {
            return HashMap::new();
        }
        let object_handle = object_handle.unwrap();
        let property_table_handle_opt = object_handle.first_child_by_key("Properties70");
        let mut properties = HashMap::new();
        if let Some(property_table_handle) = property_table_handle_opt {
            for property_detail in property_table_handle.children() {
                let r: Result<PropertyDetails, _> = property_detail.try_into();
                if let Ok(property_details) = r {
                    properties.insert(property_details.name, property_details.property);
                }
            }
        }
        properties
    }

    pub fn attributes(&self) -> HashMap<String, ElementAttribute> {
        let object_index = self.object.element_index;
        let object_handle = self
            .document
            .object_element_amphitheatre
            .get_handle(object_index);
        if object_handle.is_none() {
            return HashMap::new();
        }
        let object_handle = object_handle.unwrap();
        let mut attributes = HashMap::new();
        for attribute in object_handle.children() {
            if attribute.key() == "Properties70" {
                continue;
            }
            let subtree = self.document.object_element_amphitheatre.extract_subtree(attribute.index());
            if subtree.is_none() {
                continue;
            }
            let subtree = subtree.unwrap();
            attributes.insert(attribute.key().to_string(), subtree);
        }
        attributes
    }
}

/// OwnedObject is an object with its properties extracted from the document.
/// This is useful for accessing the object's properties and attributes
/// without having to search the document for the object's properties and attributes.
#[derive(Debug, PartialEq)]
pub struct OwnedObject {
    pub name: String,
    pub type_name: String,
    pub class_name: String,
    pub properties: HashMap<String, Property>,
    pub attributes: HashMap<String, ElementAttribute>,
    /// `OO` destinations from this object (same as [`Object::connected_object_ids`]).
    pub connected_object_ids: Vec<u64>,
    /// `OP` targets from this object (same as [`Object::object_property_targets`]).
    pub object_property_targets: Vec<ObjectPropertyConnection>,
    /// For each source property name on this object, the first `PP` destination
    /// ([`Object::pp_targets`]) when multiple targets exist for that property.
    pub pp_property_targets: HashMap<String, ObjectPropertyConnection>,
}

impl<'a> From<Object<'a>> for OwnedObject {
    fn from(object: Object<'a>) -> Self {
        let idx = object.object_index();
        let mut pp_property_targets = HashMap::new();
        for source_property in object.pp_source_property_names() {
            let Some(targets) = object.pp_targets((idx, source_property.as_str())) else {
                continue;
            };
            if let Some(first) = targets.first() {
                pp_property_targets.insert(source_property.clone(), first.clone());
            }
        }

        Self {
            name: object.name().to_string(),
            type_name: object.type_name().to_string(),
            class_name: object.class_name().to_string(),
            properties: object.properties(),
            attributes: object.attributes(),
            connected_object_ids: object.connected_object_ids().to_vec(),
            object_property_targets: object.object_property_targets().to_vec(),
            pp_property_targets,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Objects<'a> {
    /// Internal buffer iterator.
    pub(crate) iter: std::collections::hash_map::Iter<'a, u64, LazyObject>,

    /// The internal root document.
    pub(crate) document: &'a Document,
}


impl ExactSizeIterator for Objects<'_> {}
impl<'a> Iterator for Objects<'a> {
    type Item = Result<Object<'a>, ObjectError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, object)| {
            self.document.template_for_object(object)
                .map(|template| Object::new(self.document, template, object, *index))
                .ok_or_else(|| ObjectError::MissingTemplate(object.type_name.clone()))
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter.last().map(|(index, object)| {
            document.template_for_object(object)
                .map(|template| Object::new(document, template, object, *index))
                .ok_or_else(|| ObjectError::MissingTemplate(object.type_name.clone()))
        })
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|(index, object)| {
            self.document.template_for_object(object)
                .map(|template| Object::new(self.document, template, object, *index))
                .ok_or_else(|| ObjectError::MissingTemplate(object.type_name.clone()))
        })
    }
}
