use json::{StringIndex, material::ParameterValue};

use crate::{document::Document, texture::Texture};

#[derive(Clone, Debug)]
pub enum TexProperty<'a> {
    Texture(Texture<'a>),
    Color([f32; 4]),
}

impl Default for TexProperty<'_> {
    fn default() -> Self {
        TexProperty::Color([0.0, 0.0, 0.0, 1.0])
    }
}

#[derive(Clone, Debug)]
pub struct Technique<'a> {
    #[allow(dead_code)]
    document: &'a Document,

    index: &'a String,

    json: &'a json::material::Technique,
}

#[derive(Clone, Debug)]
pub struct Material<'a> {
    document: &'a Document,

    index: &'a String,

    json: &'a json::material::Material,

    technique: Technique<'a>,
}

#[derive(Clone, Debug)]
pub struct Techniques<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Technique>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

#[derive(Clone, Debug)]
pub struct Materials<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Material>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> Technique<'a> {
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::material::Technique,
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
}

impl<'a> Material<'a> {
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::material::Material,
    ) -> Self {
        let technique = document
            .techniques()
            .find(|x| Some(StringIndex::new(x.index().to_string())) == json.technique)
            .unwrap();

        Self {
            document,
            index,
            json,
            technique,
        }
    }

    pub fn index(&self) -> &str {
        self.index
    }
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }
    pub fn technique(&self) -> &Technique<'a> {
        &self.technique
    }
    pub fn double_sided(&self) -> bool {
        self.json
            .values
            .get("doubleSided")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::Boolean(b)) => Some(*b),
                _ => None,
            })
            .unwrap_or(false)
    }
    pub fn transparent(&self) -> bool {
        self.json
            .values
            .get("transparent")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::Boolean(b)) => Some(*b),
                _ => None,
            })
            .unwrap_or(false)
    }
    pub fn transparency(&self) -> f32 {
        self.json
            .values
            .get("transparency")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::Number(b)) => Some(*b),
                _ => None,
            })
            .unwrap_or(1.0)
    }
    pub fn shininess(&self) -> f32 {
        self.json
            .values
            .get("shininess")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::Number(b)) => Some(*b),
                _ => None,
            })
            .unwrap_or(0.0)
    }
    pub fn ambient(&self) -> TexProperty<'a> {
        self.json
            .values
            .get("ambient")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::NumberArray(b))
                    if b.len() == 4 =>
                {
                    Some(TexProperty::Color([b[0], b[1], b[2], b[3]]))
                }
                json::validation::Checked::Valid(ParameterValue::String(str)) => self
                    .document
                    .textures()
                    .find(|x| x.index() == str)
                    .map(TexProperty::Texture),
                _ => None,
            })
            .unwrap_or_default()
    }
    pub fn diffuse(&self) -> TexProperty<'a> {
        self.json
            .values
            .get("diffuse")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::NumberArray(b))
                    if b.len() == 4 =>
                {
                    Some(TexProperty::Color([b[0], b[1], b[2], b[3]]))
                }
                json::validation::Checked::Valid(ParameterValue::String(str)) => self
                    .document
                    .textures()
                    .find(|x| x.index() == str)
                    .map(TexProperty::Texture),
                _ => None,
            })
            .unwrap_or_default()
    }
    pub fn specular(&self) -> TexProperty<'a> {
        self.json
            .values
            .get("specular")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::NumberArray(b))
                    if b.len() == 4 =>
                {
                    Some(TexProperty::Color([b[0], b[1], b[2], b[3]]))
                }
                json::validation::Checked::Valid(ParameterValue::String(str)) => self
                    .document
                    .textures()
                    .find(|x| x.index() == str)
                    .map(TexProperty::Texture),
                _ => None,
            })
            .unwrap_or_default()
    }
    pub fn emission(&self) -> TexProperty<'a> {
        self.json
            .values
            .get("emission")
            .and_then(|x| match x {
                json::validation::Checked::Valid(ParameterValue::NumberArray(b))
                    if b.len() == 4 =>
                {
                    Some(TexProperty::Color([b[0], b[1], b[2], b[3]]))
                }
                json::validation::Checked::Valid(ParameterValue::String(str)) => self
                    .document
                    .textures()
                    .find(|x| x.index() == str)
                    .map(TexProperty::Texture),
                _ => None,
            })
            .unwrap_or_default()
    }
}

impl ExactSizeIterator for Materials<'_> {}
impl<'a> Iterator for Materials<'a> {
    type Item = Material<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Material::new(self.document, index, json))
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
            .map(|(index, json)| Material::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Material::new(self.document, index, json))
    }
}

impl ExactSizeIterator for Techniques<'_> {}
impl<'a> Iterator for Techniques<'a> {
    type Item = Technique<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Technique::new(self.document, index, json))
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
            .map(|(index, json)| Technique::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Technique::new(self.document, index, json))
    }
}
