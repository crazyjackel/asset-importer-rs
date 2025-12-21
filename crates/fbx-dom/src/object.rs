use crate::document::{Document, LazyObject, Property, PropertyDetails, Template};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Object<'a> {
    document: &'a Document,
    template: &'a Template,
    object: &'a LazyObject,
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
}
