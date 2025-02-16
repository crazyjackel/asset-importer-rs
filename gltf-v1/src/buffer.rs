use std::ops;

use json::buffer::{BufferType, BufferViewType};

use crate::document::Document;

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug)]
pub struct Buffer<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::Buffer,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug)]
pub struct View<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::BufferView,

    parent: Buffer<'a>,
}

/// Describes a buffer data source.
#[derive(Clone, Debug)]
pub enum Source<'a> {
    /// Buffer data is contained in the `BIN` section of binary glTF.
    Bin,

    /// Buffer data is contained in an external data source.
    Uri(&'a str),
}

#[derive(Clone, Debug)]
pub struct Data(pub Vec<u8>);

impl ops::Deref for Data {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> Buffer<'a> {
    /// Constructs a `Buffer`.
    pub(crate) fn new(document: &'a Document, index: &'a String, json: &'a json::Buffer) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> &str {
        self.index
    }

    pub fn source(&self) -> Source<'a> {
        if self.index == "binary_glTF" {
            Source::Bin
        } else {
            Source::Uri(&self.json.uri)
        }
    }

    pub fn length(&self) -> usize {
        self.json.byte_length.0 as usize
    }

    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    pub fn target(&self) -> Option<BufferType> {
        self.json.type_.map(|x| x.unwrap())
    }
}

impl<'a> View<'a> {
    /// Constructs a `View`.
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::BufferView,
    ) -> Self {
        let parent = document
            .buffers()
            .find(|x| x.index == json.buffer.value())
            .unwrap();
        Self {
            document,
            index,
            json,
            parent,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> &str {
        self.index
    }

    pub fn buffer(&self) -> Buffer<'a> {
        self.document
            .buffers()
            .find(|x| x.index == self.json.buffer.value())
            .unwrap()
    }

    pub fn length(&self) -> usize {
        self.json.byte_length.0 as usize
    }
    pub fn offset(&self) -> usize {
        self.json.byte_offset.0 as usize
    }
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    pub fn target(&self) -> Option<BufferViewType> {
        self.json.target.map(|target| target.unwrap())
    }
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Buffers<'a> {
    /// Internal buffer iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Buffer>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Buffers<'a> {}
impl<'a> Iterator for Buffers<'a> {
    type Item = Buffer<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Buffer::new(self.document, index, json))
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
            .map(|(index, json)| Buffer::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Buffer::new(self.document, index, json))
    }
}

/// An `Iterator` that visits every buffer view in a glTF asset.
#[derive(Clone, Debug)]
pub struct Views<'a> {
    /// Internal buffer view iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::BufferView>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Views<'a> {}
impl<'a> Iterator for Views<'a> {
    type Item = View<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| View::new(self.document, index, json))
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
            .map(|(index, json)| View::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| View::new(self.document, index, json))
    }
}
