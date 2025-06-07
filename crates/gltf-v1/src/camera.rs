use json::camera::CameraType;

use crate::Document;

/// A camera's projection.
#[derive(Clone, Debug)]
pub enum Projection<'a> {
    /// Describes an orthographic projection.
    Orthographic(Orthographic<'a>),

    /// Describes a perspective projection.
    Perspective(Perspective<'a>),
}

///  Values for an orthographic camera projection.
#[derive(Clone, Debug)]
pub struct Orthographic<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::camera::Orthographic,
}

/// Values for a perspective camera projection.
#[derive(Clone, Debug)]
pub struct Perspective<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON struct.
    json: &'a json::camera::Perspective,
}

#[derive(Clone, Debug)]
pub struct Camera<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::Camera,
}

impl<'a> Camera<'a> {
    /// Constructs a `Buffer`.
    pub(crate) fn new(document: &'a Document, index: &'a String, json: &'a json::Camera) -> Self {
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

    /// Returns the camera's projection.
    pub fn projection(&self) -> Projection {
        match self.json.type_.unwrap() {
            CameraType::Orthographic => {
                let json = self.json.orthographic.as_ref().unwrap();
                Projection::Orthographic(Orthographic::new(self.document, json))
            }
            CameraType::Perspective => {
                let json = self.json.perspective.as_ref().unwrap();
                Projection::Perspective(Perspective::new(self.document, json))
            }
        }
    }
}
impl<'a> Orthographic<'a> {
    /// Constructs a `Orthographic` camera projection.
    pub(crate) fn new(document: &'a Document, json: &'a json::camera::Orthographic) -> Self {
        Self { document, json }
    }

    ///  The horizontal magnification of the view.
    pub fn xmag(&self) -> f32 {
        self.json.xmag
    }

    ///  The vertical magnification of the view.
    pub fn ymag(&self) -> f32 {
        self.json.ymag
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> f32 {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }
}

impl<'a> Perspective<'a> {
    /// Constructs a `Perspective` camera projection.
    pub(crate) fn new(document: &'a Document, json: &'a json::camera::Perspective) -> Self {
        Self { document, json }
    }

    ///  Aspect ratio of the field of view.
    pub fn aspect_ratio(&self) -> Option<f32> {
        self.json.aspect_ratio
    }

    ///  The vertical field of view in radians.
    pub fn yfov(&self) -> f32 {
        self.json.yfov
    }

    ///  The distance to the far clipping plane.
    pub fn zfar(&self) -> f32 {
        self.json.zfar
    }

    ///  The distance to the near clipping plane.
    pub fn znear(&self) -> f32 {
        self.json.znear
    }
}

/// An `Iterator` that visits every buffer in a glTF asset.
#[derive(Clone, Debug)]
pub struct Cameras<'a> {
    /// Internal buffer iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Camera>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl ExactSizeIterator for Cameras<'_> {}
impl<'a> Iterator for Cameras<'a> {
    type Item = Camera<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Camera::new(self.document, index, json))
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
            .map(|(index, json)| Camera::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Camera::new(self.document, index, json))
    }
}
