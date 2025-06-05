use std::slice;

use json::StringIndex;

use crate::{
    Document,
    camera::Camera,
    light::Light,
    math::{Matrix3, Matrix4, Quaternion, Vector3},
    mesh::Mesh,
    skin::Skin,
};

/// The transform for a `Node`.
#[derive(Clone, Debug)]
pub enum Transform {
    /// 4x4 transformation matrix in column-major order.
    Matrix {
        /// 4x4 matrix.
        matrix: [[f32; 4]; 4],
    },

    /// Decomposed TRS properties.
    Decomposed {
        /// `[x, y, z]` vector.
        translation: [f32; 3],

        /// `[x, y, z, w]` quaternion, where `w` is the scalar.
        rotation: [f32; 4],

        /// `[x, y, z]` vector.
        scale: [f32; 3],
    },
}
impl Transform {
    pub fn matrix(self) -> [[f32; 4]; 4] {
        match self {
            Transform::Matrix { matrix } => matrix,
            Transform::Decomposed {
                translation: t,
                rotation: r,
                scale: s,
            } => {
                let t = Matrix4::from_translation(Vector3::new(t[0], t[1], t[2]));
                let r = Matrix4::from_quaternion(Quaternion::new(r[3], r[0], r[1], r[2]));
                let s = Matrix4::from_nonuniform_scale(s[0], s[1], s[2]);
                (t * r * s).as_array()
            }
        }
    }

    pub fn decomposed(self) -> ([f32; 3], [f32; 4], [f32; 3]) {
        match self {
            Transform::Matrix { matrix: m } => {
                let translation = [m[3][0], m[3][1], m[3][2]];
                #[rustfmt::skip]
                let mut i = Matrix3::new(
                    m[0][0], m[0][1], m[0][2],
                    m[1][0], m[1][1], m[1][2],
                    m[2][0], m[2][1], m[2][2],
                );
                let sx = i.x.magnitude();
                let sy = i.y.magnitude();
                let sz = i.determinant().signum() * i.z.magnitude();
                let scale = [sx, sy, sz];
                i.x.multiply(1.0 / sx);
                i.y.multiply(1.0 / sy);
                i.z.multiply(1.0 / sz);
                let r = Quaternion::from_matrix(i);
                let rotation = [r.v.x, r.v.y, r.v.z, r.s];
                (translation, rotation, scale)
            }
            Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => (translation, rotation, scale),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: &'a String,

    /// The corresponding JSON struct.
    json: &'a json::Node,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub(crate) fn new(document: &'a Document, index: &'a String, json: &'a json::Node) -> Self {
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
    pub fn camera(&self) -> Option<Camera<'a>> {
        self.json
            .camera
            .as_ref()
            .and_then(|index| self.document.cameras().find(|x| x.index() == index.value()))
    }
    pub fn light(&self) -> Option<Light<'a>> {
        #[cfg(feature = "KHR_materials_common")]
        return self
            .json
            .extensions
            .as_ref()
            .and_then(|x| x.ktr_materials_common.as_ref())
            .and_then(|node_light| {
                self.document
                    .lights()
                    .and_then(|mut l| l.find(|x| x.index() == node_light.light.value()))
            });
        #[cfg(not(feature = "KHR_materials_common"))]
        None
    }
    pub fn children(&self) -> Children<'a> {
        Children {
            document: self.document,
            iter: self.json.children.iter(),
        }
    }
    pub fn skeletons(&self) -> Children<'a> {
        Children {
            document: self.document,
            iter: self.json.skeletons.iter(),
        }
    }
    pub fn skin(&self) -> Option<Skin<'a>> {
        self.json
            .skin
            .as_ref()
            .and_then(|index| self.document.skins().find(|x| x.index() == index.value()))
    }
    pub fn meshes(&self) -> Vec<Mesh<'a>> {
        self.json
            .meshes
            .iter()
            .filter_map(|index| self.document.meshes().find(|x| x.index() == index.value()))
            .collect()
    }
    pub fn joint_name(&self) -> Option<&'a str> {
        self.json.joint_name.as_deref()
    }
    /// Returns the node's transform.
    pub fn transform(&self) -> Transform {
        if let Some(m) = self.json.matrix {
            Transform::Matrix {
                matrix: [
                    [m[0], m[1], m[2], m[3]],
                    [m[4], m[5], m[6], m[7]],
                    [m[8], m[9], m[10], m[11]],
                    [m[12], m[13], m[14], m[15]],
                ],
            }
        } else {
            Transform::Decomposed {
                translation: self.json.translation.unwrap_or([0.0, 0.0, 0.0]),
                rotation: self.json.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                scale: self.json.scale.unwrap_or([1.0, 1.0, 1.0]),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Children<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The internal node index iterator.
    pub(crate) iter: slice::Iter<'a, StringIndex<json::Node>>,
}

impl<'a> ExactSizeIterator for Children<'a> {}
impl<'a> Iterator for Children<'a> {
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
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// Internal accessor iterator.
    pub(crate) iter: indexmap::map::Iter<'a, String, gltf_v1_json::Node>,

    /// The internal root glTF object.
    pub(crate) document: &'a Document,
}

impl<'a> ExactSizeIterator for Nodes<'a> {}
impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(index, json)| Node::new(self.document, index, json))
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
            .map(|(index, json)| Node::new(document, index, json))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter
            .nth(n)
            .map(|(index, json)| Node::new(self.document, index, json))
    }
}
