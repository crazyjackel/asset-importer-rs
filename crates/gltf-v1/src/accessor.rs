use std::{iter, slice};

use json::accessor::{ComponentType, Type};

use crate::{buffer::View, document::Document};

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,

    view: View<'a>,
}

impl<'a> Accessor<'a> {
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::accessor::Accessor,
    ) -> Self {
        let view = document
            .views()
            .find(|x| x.index() == json.buffer_view.value())
            .unwrap();
        Self {
            document,
            index,
            json,
            view,
        }
    }
    pub fn index(&self) -> &str {
        self.index
    }
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }
    pub fn view(&self) -> &View<'a> {
        &self.view
    }
    pub fn count(&self) -> usize {
        self.json.count as usize
    }
    pub fn offset(&self) -> usize {
        self.json.byte_offset as usize
    }
    pub fn stride(&self) -> Option<usize> {
        self.json.byte_stride.map(|x| x as usize)
    }
    pub fn component_type(&self) -> ComponentType {
        self.json.component_type.unwrap()
    }
    pub fn accessor_type(&self) -> Type {
        self.json.type_.unwrap()
    }
}

/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Clone, Debug)]
pub struct Accessors<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Accessor>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Accessors<'a> {}
impl<'a> Iterator for Accessors<'a> {
    type Item = Accessor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Accessor::new(self.document, index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        let document = self.document;
        self.iter
            .last()
            .map(|(index, json)| Accessor::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Accessor::new(self.document, index, json))
    }
}
