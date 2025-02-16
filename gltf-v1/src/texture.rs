use crate::{document::Document, image::Image};

#[derive(Clone, Debug)]
pub struct Sampler<'a> {
    document: &'a Document,

    index: &'a String,

    json: &'a json::texture::Sampler,
}

#[derive(Clone, Debug)]
pub struct Texture<'a> {
    document: &'a Document,

    index: &'a String,

    json: &'a json::texture::Texture,
}

impl<'a> Sampler<'a> {
    /// Constructs a `Sampler`.
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::texture::Sampler,
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
}

impl<'a> Texture<'a> {
    /// Constructs a `Texture`.
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::texture::Texture,
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
    pub fn source(&self) -> Image<'a> {
        self.document
            .images()
            .find(|x| x.index() == self.json.source.value())
            .unwrap()
    }
}

/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Clone, Debug)]
pub struct Textures<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Texture>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Textures<'a> {}
impl<'a> Iterator for Textures<'a> {
    type Item = Texture<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Texture::new(self.document, index, json))
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
            .map(|(index, json)| Texture::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Texture::new(self.document, index, json))
    }
}

#[derive(Clone, Debug)]
pub struct Samplers<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Sampler>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Samplers<'a> {}
impl<'a> Iterator for Samplers<'a> {
    type Item = Sampler<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Sampler::new(self.document, index, json))
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
            .map(|(index, json)| Sampler::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Sampler::new(self.document, index, json))
    }
}
