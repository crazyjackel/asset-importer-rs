use std::{iter, slice};

use json::{
    mesh::{PrimitiveMode, Semantic},
    validation::Checked,
};

use crate::{Accessor, Document, material::Material};

pub type Attribute<'a> = (Checked<Semantic>, Accessor<'a>);

#[derive(Clone, Debug)]
pub struct Attributes<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The parent `Primitive` struct.
    #[allow(dead_code)]
    pub(crate) prim: Primitive<'a>,

    /// The internal attribute iterator.
    pub(crate) iter:
        indexmap::map::Iter<'a, Checked<Semantic>, gltf_v1_json::StringIndex<json::Accessor>>,
}

#[derive(Clone, Debug)]
pub struct Primitive<'a> {
    /// The parent `Mesh` struct.
    mesh: Mesh<'a>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub(crate) fn new(mesh: Mesh<'a>, index: usize, json: &'a json::mesh::Primitive) -> Self {
        Self { mesh, index, json }
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn mode(&self) -> PrimitiveMode {
        self.json.mode.unwrap()
    }
    pub fn get(&self, semantic: &Semantic) -> Option<Accessor<'a>> {
        self.json
            .attributes
            .get(&Checked::Valid(*semantic))
            .and_then(|x| {
                self.mesh
                    .document
                    .accessors()
                    .find(|y| y.index() == x.value())
            })
    }
    pub fn indices(&self) -> Option<Accessor<'a>> {
        self.json.indices.as_ref().and_then(|x| {
            self.mesh
                .document
                .accessors()
                .find(|y| y.index() == x.value())
        })
    }
    pub fn material(&self) -> Material<'a> {
        self.mesh
            .document
            .materials()
            .find(|x| x.index() == self.json.material.value())
            .unwrap()
    }
    pub fn attributes(&self) -> Attributes<'a> {
        Attributes {
            document: self.mesh.document,
            prim: self.clone(),
            iter: self.json.attributes.iter(),
        }
    }
}
/// A set of primitives to be rendered.
#[derive(Clone, Debug)]
pub struct Mesh<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

impl<'a> Mesh<'a> {
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::mesh::Mesh,
    ) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    pub fn index(&self) -> &str {
        self.index
    }

    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    pub fn primitives(&self) -> Primitives<'a> {
        Primitives {
            mesh: self.clone(),
            iter: self.json.primitives.iter().enumerate(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Meshes<'a> {
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Mesh>,

    pub(crate) document: &'a Document,
}

impl ExactSizeIterator for Meshes<'_> {}
impl<'a> Iterator for Meshes<'a> {
    type Item = Mesh<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Mesh::new(self.document, index, json))
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
            .map(|(index, json)| Mesh::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Mesh::new(self.document, index, json))
    }
}

#[derive(Clone, Debug)]
pub struct Primitives<'a> {
    /// The parent `Mesh` struct.
    pub(crate) mesh: Mesh<'a>,

    /// The internal JSON primitive iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::mesh::Primitive>>,
}
impl ExactSizeIterator for Primitives<'_> {}
impl<'a> Iterator for Primitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Primitive::new(self.mesh.clone(), index, json))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn last(mut self) -> Option<Self::Item> {
        let mesh = self.mesh;
        self.iter
            .next_back()
            .map(|(index, json)| Primitive::new(mesh, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Primitive::new(self.mesh.clone(), index, json))
    }
}

impl ExactSizeIterator for Attributes<'_> {}
impl<'a> Iterator for Attributes<'a> {
    type Item = Attribute<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, index)| {
            let semantic = *key;
            let accessor = self
                .document
                .accessors()
                .find(|x| x.index() == index.value())
                .unwrap();
            (semantic, accessor)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
