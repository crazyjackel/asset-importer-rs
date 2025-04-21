use json::extensions::light::Type;

use crate::Document;

#[derive(Clone, Debug)]
pub struct Light<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::extensions::light::Light,
}

impl<'a> Light<'a> {
    /// Constructs a `Buffer`.
    pub(crate) fn new(
        document: &'a Document,
        index: &'a String,
        json: &'a json::extensions::light::Light,
    ) -> Self {
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
    pub fn color(&self) -> [f32; 4] {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Ambient) => {
                self.json.ambient.as_ref().unwrap().color
            }
            json::validation::Checked::Valid(Type::Directional) => {
                self.json.directional.as_ref().unwrap().color
            }
            json::validation::Checked::Valid(Type::Point) => {
                self.json.point.as_ref().unwrap().color
            }
            json::validation::Checked::Valid(Type::Spot) => self.json.spot.as_ref().unwrap().color,
            _ => [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn constant_attenuation(&self) -> f32 {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Point) => {
                self.json.point.as_ref().unwrap().constant_attenuation
            }
            json::validation::Checked::Valid(Type::Spot) => {
                self.json.spot.as_ref().unwrap().constant_attenuation
            }
            _ => 0.0,
        }
    }
    pub fn linear_attenuation(&self) -> f32 {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Point) => {
                self.json.point.as_ref().unwrap().linear_attenuation
            }
            json::validation::Checked::Valid(Type::Spot) => {
                self.json.spot.as_ref().unwrap().linear_attenuation
            }
            _ => 1.0,
        }
    }
    pub fn quadratic_attenuation(&self) -> f32 {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Point) => {
                self.json.point.as_ref().unwrap().quadratic_attenuation
            }
            json::validation::Checked::Valid(Type::Spot) => {
                self.json.spot.as_ref().unwrap().quadratic_attenuation
            }
            _ => 1.0,
        }
    }
    pub fn distance(&self) -> f32 {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Point) => {
                self.json.point.as_ref().unwrap().distance
            }
            json::validation::Checked::Valid(Type::Spot) => {
                self.json.spot.as_ref().unwrap().distance
            }
            _ => 0.0,
        }
    }
    pub fn falloff_angle(&self) -> f32 {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Spot) => {
                self.json.spot.as_ref().unwrap().falloff_angle
            }
            _ => std::f32::consts::PI / 2.0,
        }
    }
    pub fn falloff_exponent(&self) -> f32 {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Spot) => {
                self.json.spot.as_ref().unwrap().falloff_exponent
            }
            _ => 0.0,
        }
    }
    pub fn kind(&self) -> Kind {
        match self.json.type_ {
            json::validation::Checked::Valid(Type::Ambient) => Kind::Ambient,
            json::validation::Checked::Valid(Type::Directional) => Kind::Directional,
            json::validation::Checked::Valid(Type::Point) => Kind::Point,
            json::validation::Checked::Valid(Type::Spot) => Kind::Spot,
            _ => Kind::Ambient,
        }
    }
}

pub enum Kind {
    Ambient,
    Directional,
    Point,
    Spot,
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Lights<'a> {
    /// Internal buffer iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::extensions::Light>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Lights<'a> {}
impl<'a> Iterator for Lights<'a> {
    type Item = Light<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Light::new(self.document, index, json))
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
            .map(|(index, json)| Light::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Light::new(self.document, index, json))
    }
}
