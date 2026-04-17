use fbxscii::ElementAttribute;

use crate::document::{Document, LazyObject, Property, PropertyDetails, Template};
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

    pub fn element(&self) -> Option<&'a fbxscii::Element> {
        self.document.object_element_amphitheatre.get(self.object.element_index)
    }
}

impl<'a> Object<'a> {
    pub fn new(document: &'a Document, template: &'a Template, object: &'a LazyObject) -> Self {
        Self {
            document,
            template,
            object,
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
}

impl<'a> From<Object<'a>> for OwnedObject {
    fn from(object: Object<'a>) -> Self {
        Self {
            name: object.name().to_string(),
            type_name: object.type_name().to_string(),
            class_name: object.class_name().to_string(),
            properties: object.properties(),
            attributes: object.attributes(),
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

fn template_for_object<'a>(
    document: &'a Document,
    object: &'a LazyObject,
) -> Option<&'a Template> {
    document
        .templates
        .get(&object.type_name)
        .or_else(|| {
            document
                .default_template_by_object_type
                .get(&object.type_name)
                .and_then(|full_key| document.templates.get(full_key))
        })
}

impl ExactSizeIterator for Objects<'_> {}
impl<'a> Iterator for Objects<'a> {
    type Item = Result<Object<'a>, ObjectError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_index, object)| {
            template_for_object(self.document, object)
                .map(|template| Object::new(self.document, template, object))
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
        self.iter.last().map(|(_index, object)| {
            template_for_object(document, object)
                .map(|template| Object::new(document, template, object))
                .ok_or_else(|| ObjectError::MissingTemplate(object.type_name.clone()))
        })
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|(_index, object)| {
            template_for_object(self.document, object)
                .map(|template| Object::new(self.document, template, object))
                .ok_or_else(|| ObjectError::MissingTemplate(object.type_name.clone()))
        })
    }
}
