use std::{iter, slice};

use json::StringIndex;

use crate::{
    Document,
    node::{Node, Nodes},
};

#[derive(Clone, Debug)]
pub struct Scene<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::Scene,
}

impl<'a> Scene<'a> {
    /// Constructs a `Buffer`.
    pub(crate) fn new(document: &'a Document, index: &'a String, json: &'a json::Scene) -> Self {
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
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    pub fn nodes(&self) -> SceneNodes<'a> {
        SceneNodes {
            document: self.document,
            iter: self.json.nodes.iter(),
        }
    }
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Scenes<'a> {
    /// Internal buffer iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Scene>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

#[derive(Clone, Debug)]
pub struct SceneNodes<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: slice::Iter<'a, StringIndex<json::node::Node>>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for SceneNodes<'a> {}
impl<'a> Iterator for SceneNodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|index| self.document.nodes().find(|x| x.index() == index.value()))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(self) -> Option<Self::Item> {
        self.iter
            .last()
            .and_then(|index| self.document.nodes().find(|x| x.index() == index.value()))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .and_then(|index| self.document.nodes().find(|x| x.index() == index.value()))
    }
}

impl<'a> ExactSizeIterator for Scenes<'a> {}
impl<'a> Iterator for Scenes<'a> {
    type Item = Scene<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Scene::new(self.document, index, json))
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
            .map(|(index, json)| Scene::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Scene::new(self.document, index, json))
    }
}
