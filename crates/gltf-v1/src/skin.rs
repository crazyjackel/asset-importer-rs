use crate::Document;

#[derive(Clone, Debug)]
pub struct Skin<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::skin::Skin,
}

impl<'a> Skin<'a> {
    /// Constructs a `Skin`.
    pub(crate) fn new(document: &'a Document, index: &'a String, json: &'a json::Skin) -> Self {
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
}
#[derive(Clone, Debug)]
pub struct Skins<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Skin>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Skins<'a> {}
impl<'a> Iterator for Skins<'a> {
    type Item = Skin<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Skin::new(self.document, index, json))
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
            .map(|(index, json)| Skin::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Skin::new(self.document, index, json))
    }
}
