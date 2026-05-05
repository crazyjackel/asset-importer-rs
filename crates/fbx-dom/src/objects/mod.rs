//! Typed FBX object wrappers aligned with Assimp’s [`FBXDocument`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.h)
//! and [`LazyObject::Get`](https://github.com/assimp/assimp/blob/master/code/AssetLib/FBX/FBXDocument.cpp)-style dispatch.
//!
//! ## Dispatch (`fbx_object_tag` / [`ClassifiedFbxObject`])
//!
//! - **`type_name`** is the FBX object class family (`Geometry`, `Model`, `Material`, …).
//! - **`class_name`** narrows within the family (`Mesh`, `Camera`, `Skin`, …).
//! - Materials, textures, video, and animation types are keyed by **`type_name` only** so varied
//!   SDK `class_name` strings still classify.
//! - Unknown `Geometry` / `NodeAttribute` / `Deformer` classes become explicit `Unknown*` variants
//!   instead of the generic [`ClassifiedFbxObject::Unknown`].
//!
//! Each concrete type is a newtype over [`crate::OwnedObject`] (or a small struct) with
//! [`TryFrom<OwnedObject>`] for narrowing. Prefer [`ClassifiedFbxObject::try_from`] when the kind is
//! not known upfront.

mod animation_curve;
mod animation_curve_node;
mod animation_layer;
mod animation_stack;
mod blend_shape;
mod blend_shape_channel;
mod camera;
mod camera_switcher;
mod cluster;
mod extract;
mod global_settings;
mod layered_texture;
mod light;
mod limb_node;
mod line_geometry;
mod material;
mod mesh_geometry;
mod model;
mod null_node;
mod shape_geometry;
mod skin;
mod texture;
mod video;

pub use animation_curve::AnimationCurve;
pub use animation_curve_node::AnimationCurveNode;
pub use animation_layer::AnimationLayer;
pub use animation_stack::AnimationStack;
pub use blend_shape::BlendShape;
pub use blend_shape_channel::BlendShapeChannel;
pub use camera::Camera;
pub use camera_switcher::CameraSwitcher;
pub use cluster::Cluster;
pub use extract::AttrExtractor;
pub use extract::AttrExtractorExt;
pub use extract::AttrExtractorParseExt;
pub use global_settings::OwnedGlobalSettings;
pub use layered_texture::LayeredTexture;
pub use light::Light;
pub use light::LightDecay;
pub use light::LightType;
pub use limb_node::LimbNode;
pub use line_geometry::LineGeometry;
pub use material::Material;
pub use mesh_geometry::MeshGeometry;
pub use model::Model;
pub use model::ModelRotationOrder;
pub use model::ModelTransformInheritance;
pub use null_node::NullNode;
pub use shape_geometry::ShapeGeometry;
pub use skin::Skin;
pub use texture::Texture;
pub use video::Video;

use crate::OwnedObject;

/// Borrowed polymorphic NodeAttribute reference (replacement for inheritance-style base).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeAttributeRef<'a> {
    Camera(&'a Camera),
    CameraSwitcher(&'a CameraSwitcher),
    Light(&'a Light),
    NullNode(&'a NullNode),
    LimbNode(&'a LimbNode),
    Unknown(&'a OwnedObject),
}

impl<'a> NodeAttributeRef<'a> {
    pub fn inner(self) -> &'a OwnedObject {
        match self {
            NodeAttributeRef::Camera(x) => x.inner(),
            NodeAttributeRef::CameraSwitcher(x) => x.inner(),
            NodeAttributeRef::Light(x) => x.inner(),
            NodeAttributeRef::NullNode(x) => x.inner(),
            NodeAttributeRef::LimbNode(x) => x.inner(),
            NodeAttributeRef::Unknown(x) => x,
        }
    }
}

/// Borrowed polymorphic `Geometry` reference for incoming `Geometry -> Model` `OO` links.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModelGeometryRef<'a> {
    Mesh(&'a MeshGeometry),
    Line(&'a LineGeometry),
    Shape(&'a ShapeGeometry),
    Unknown(&'a OwnedObject),
}

impl<'a> ModelGeometryRef<'a> {
    pub fn inner(self) -> &'a OwnedObject {
        match self {
            ModelGeometryRef::Mesh(x) => x.inner(),
            ModelGeometryRef::Line(x) => x.inner(),
            ModelGeometryRef::Shape(x) => x.inner(),
            ModelGeometryRef::Unknown(x) => x,
        }
    }
}

// --- `type_name` / `class_name` pairs (Assimp `LazyObject::Get` dispatch) -----------------------

pub const MODEL_TYPE_NAME: &str = "Model";
pub const MODEL_IK_EFFECTOR_CLASS_NAME: &str = "IKEffector";
pub const MODEL_FK_EFFECTOR_CLASS_NAME: &str = "FKEffector";

pub const GEOMETRY_TYPE_NAME: &str = "Geometry";
pub const GEOMETRY_MESH_CLASS_NAME: &str = "Mesh";
pub const GEOMETRY_LINE_CLASS_NAME: &str = "Line";
pub const GEOMETRY_SHAPE_CLASS_NAME: &str = "Shape";

pub const NODE_ATTRIBUTE_TYPE_NAME: &str = "NodeAttribute";
pub const NODE_ATTRIBUTE_CAMERA_CLASS_NAME: &str = "Camera";
pub const NODE_ATTRIBUTE_CAMERA_SWITCHER_CLASS_NAME: &str = "CameraSwitcher";
pub const NODE_ATTRIBUTE_LIGHT_CLASS_NAME: &str = "Light";
pub const NODE_ATTRIBUTE_NULL_CLASS_NAME: &str = "Null";
pub const NODE_ATTRIBUTE_LIMB_NODE_CLASS_NAME: &str = "LimbNode";

pub const MATERIAL_TYPE_NAME: &str = "Material";
pub const MATERIAL_CLASS_NAME: &str = "Material";

pub const TEXTURE_TYPE_NAME: &str = "Texture";
pub const TEXTURE_CLASS_NAME: &str = "Texture";

pub const LAYERED_TEXTURE_TYPE_NAME: &str = "LayeredTexture";
pub const LAYERED_TEXTURE_CLASS_NAME: &str = "LayeredTexture";

pub const VIDEO_TYPE_NAME: &str = "Video";
pub const VIDEO_CLASS_NAME: &str = "Video";

pub const DEFORMER_TYPE_NAME: &str = "Deformer";
pub const DEFORMER_CLUSTER_CLASS_NAME: &str = "Cluster";
pub const DEFORMER_SKIN_CLASS_NAME: &str = "Skin";
pub const DEFORMER_BLEND_SHAPE_CLASS_NAME: &str = "BlendShape";
pub const DEFORMER_BLEND_SHAPE_CHANNEL_CLASS_NAME: &str = "BlendShapeChannel";

pub const ANIMATION_STACK_TYPE_NAME: &str = "AnimationStack";
pub const ANIMATION_STACK_CLASS_NAME: &str = "AnimationStack";

pub const ANIMATION_LAYER_TYPE_NAME: &str = "AnimationLayer";
pub const ANIMATION_LAYER_CLASS_NAME: &str = "AnimationLayer";

pub const ANIMATION_CURVE_TYPE_NAME: &str = "AnimationCurve";
pub const ANIMATION_CURVE_CLASS_NAME: &str = "AnimationCurve";

pub const ANIMATION_CURVE_NODE_TYPE_NAME: &str = "AnimationCurveNode";
pub const ANIMATION_CURVE_NODE_CLASS_NAME: &str = "AnimationCurveNode";

/// Why [`TryFrom`]`<`[`OwnedObject`]`>` failed for a typed FBX wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FbxTryFromReason {
    /// `(type_name, class_name)` does not match the target wrapper (factory: `FbxTypeMismatch::wrong_object_kind`).
    WrongObjectKind {
        expected: String,
        got_type_name: String,
        got_class_name: String,
    },
    /// A required non-`Properties70` child (FBX element under the object) was missing.
    MissingAttribute { name: String },
    /// A child was present but had no usable value or failed to parse.
    InvalidAttributeFormat { name: String, detail: String },
}

/// Returned when [`TryFrom`]`<`[`OwnedObject`]`>` fails for a typed FBX wrapper.
#[derive(Debug, PartialEq)]
pub struct FbxTypeMismatch {
    pub object: OwnedObject,
    pub reason: FbxTryFromReason,
}

impl FbxTypeMismatch {
    fn new(o: OwnedObject, reason: FbxTryFromReason) -> FbxTypeMismatch {
        FbxTypeMismatch { object: o, reason }
    }

    pub(crate) fn wrong_object_kind(o: OwnedObject, expected: String) -> FbxTypeMismatch {
        let reason = FbxTryFromReason::WrongObjectKind {
            expected,
            got_type_name: o.type_name.clone(),
            got_class_name: o.class_name.clone(),
        };
        FbxTypeMismatch { object: o, reason }
    }
}

/// Internal discriminant for [`fbx_object_tag`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FbxObjectTag {
    Model,
    MeshGeometry,
    LineGeometry,
    ShapeGeometry,
    UnknownGeometry,
    Camera,
    CameraSwitcher,
    Light,
    NullNode,
    LimbNode,
    UnknownNodeAttribute,
    Material,
    Texture,
    LayeredTexture,
    Video,
    Cluster,
    Skin,
    BlendShape,
    BlendShapeChannel,
    UnknownDeformer,
    AnimationStack,
    AnimationLayer,
    AnimationCurve,
    AnimationCurveNode,
    /// Unsupported `Model` kinds (e.g. IK/FK effectors) or any object not mapped above.
    Unknown,
}

/// Map [`OwnedObject::type_name`] and [`OwnedObject::class_name`] to a dispatch tag.
///
/// `Material`, `Texture`, `LayeredTexture`, `Video`, and animation objects are keyed by
/// `type_name` only (SDK exports vary `class_name`). `Geometry` / `NodeAttribute` / `Deformer`
/// use `class_name` for known Assimp kinds; anything else under those types gets an explicit
/// `Unknown*` tag instead of falling through to a generic [`FbxObjectTag::Unknown`].
pub(crate) fn fbx_object_tag(o: &OwnedObject) -> FbxObjectTag {
    let ty = o.type_name.as_str();
    let cls = o.class_name.as_str();

    match ty {
        MODEL_TYPE_NAME => {
            if cls == MODEL_IK_EFFECTOR_CLASS_NAME || cls == MODEL_FK_EFFECTOR_CLASS_NAME {
                FbxObjectTag::Unknown
            } else {
                FbxObjectTag::Model
            }
        }

        GEOMETRY_TYPE_NAME => match cls {
            GEOMETRY_MESH_CLASS_NAME => FbxObjectTag::MeshGeometry,
            GEOMETRY_LINE_CLASS_NAME => FbxObjectTag::LineGeometry,
            GEOMETRY_SHAPE_CLASS_NAME => FbxObjectTag::ShapeGeometry,
            _ => FbxObjectTag::UnknownGeometry,
        },

        NODE_ATTRIBUTE_TYPE_NAME => match cls {
            NODE_ATTRIBUTE_CAMERA_CLASS_NAME => FbxObjectTag::Camera,
            NODE_ATTRIBUTE_CAMERA_SWITCHER_CLASS_NAME => FbxObjectTag::CameraSwitcher,
            NODE_ATTRIBUTE_LIGHT_CLASS_NAME => FbxObjectTag::Light,
            NODE_ATTRIBUTE_NULL_CLASS_NAME => FbxObjectTag::NullNode,
            NODE_ATTRIBUTE_LIMB_NODE_CLASS_NAME => FbxObjectTag::LimbNode,
            _ => FbxObjectTag::UnknownNodeAttribute,
        },

        MATERIAL_TYPE_NAME => FbxObjectTag::Material,
        TEXTURE_TYPE_NAME => FbxObjectTag::Texture,
        LAYERED_TEXTURE_TYPE_NAME => FbxObjectTag::LayeredTexture,
        VIDEO_TYPE_NAME => FbxObjectTag::Video,

        DEFORMER_TYPE_NAME => match cls {
            DEFORMER_CLUSTER_CLASS_NAME => FbxObjectTag::Cluster,
            DEFORMER_SKIN_CLASS_NAME => FbxObjectTag::Skin,
            DEFORMER_BLEND_SHAPE_CLASS_NAME => FbxObjectTag::BlendShape,
            DEFORMER_BLEND_SHAPE_CHANNEL_CLASS_NAME => FbxObjectTag::BlendShapeChannel,
            _ => FbxObjectTag::UnknownDeformer,
        },

        ANIMATION_STACK_TYPE_NAME => FbxObjectTag::AnimationStack,
        ANIMATION_LAYER_TYPE_NAME => FbxObjectTag::AnimationLayer,
        ANIMATION_CURVE_TYPE_NAME => FbxObjectTag::AnimationCurve,
        ANIMATION_CURVE_NODE_TYPE_NAME => FbxObjectTag::AnimationCurveNode,

        _ => FbxObjectTag::Unknown,
    }
}

impl TryFrom<OwnedObject> for ClassifiedFbxObject {
    type Error = FbxTypeMismatch;

    fn try_from(o: OwnedObject) -> Result<Self, Self::Error> {
        match fbx_object_tag(&o) {
            FbxObjectTag::Model => Ok(ClassifiedFbxObject::Model(Model::try_from(o)?)),
            FbxObjectTag::MeshGeometry => Ok(ClassifiedFbxObject::MeshGeometry(
                MeshGeometry::try_from(o)?,
            )),
            FbxObjectTag::LineGeometry => Ok(ClassifiedFbxObject::LineGeometry(
                LineGeometry::try_from(o)?,
            )),
            FbxObjectTag::ShapeGeometry => Ok(ClassifiedFbxObject::ShapeGeometry(
                ShapeGeometry::try_from(o)?,
            )),
            FbxObjectTag::UnknownGeometry => Ok(ClassifiedFbxObject::UnknownGeometry(o)),
            FbxObjectTag::Camera => Ok(ClassifiedFbxObject::Camera(Camera::try_from(o)?)),
            FbxObjectTag::CameraSwitcher => Ok(ClassifiedFbxObject::CameraSwitcher(
                CameraSwitcher::try_from(o)?,
            )),
            FbxObjectTag::Light => Ok(ClassifiedFbxObject::Light(Light::try_from(o)?)),
            FbxObjectTag::NullNode => Ok(ClassifiedFbxObject::NullNode(NullNode::try_from(o)?)),
            FbxObjectTag::LimbNode => Ok(ClassifiedFbxObject::LimbNode(LimbNode::try_from(o)?)),
            FbxObjectTag::UnknownNodeAttribute => Ok(ClassifiedFbxObject::UnknownNodeAttribute(o)),
            FbxObjectTag::Material => Ok(ClassifiedFbxObject::Material(Material::try_from(o)?)),
            FbxObjectTag::Texture => Ok(ClassifiedFbxObject::Texture(Texture::try_from(o)?)),
            FbxObjectTag::LayeredTexture => Ok(ClassifiedFbxObject::LayeredTexture(
                LayeredTexture::try_from(o)?,
            )),
            FbxObjectTag::Video => Ok(ClassifiedFbxObject::Video(Video::try_from(o)?)),
            FbxObjectTag::Cluster => Ok(ClassifiedFbxObject::Cluster(Cluster::try_from(o)?)),
            FbxObjectTag::Skin => Ok(ClassifiedFbxObject::Skin(Skin::try_from(o)?)),
            FbxObjectTag::BlendShape => {
                Ok(ClassifiedFbxObject::BlendShape(BlendShape::try_from(o)?))
            }
            FbxObjectTag::BlendShapeChannel => Ok(ClassifiedFbxObject::BlendShapeChannel(
                BlendShapeChannel::try_from(o)?,
            )),
            FbxObjectTag::UnknownDeformer => Ok(ClassifiedFbxObject::UnknownDeformer(o)),
            FbxObjectTag::AnimationStack => Ok(ClassifiedFbxObject::AnimationStack(
                AnimationStack::try_from(o)?,
            )),
            FbxObjectTag::AnimationLayer => Ok(ClassifiedFbxObject::AnimationLayer(
                AnimationLayer::try_from(o)?,
            )),
            FbxObjectTag::AnimationCurve => Ok(ClassifiedFbxObject::AnimationCurve(
                AnimationCurve::try_from(o)?,
            )),
            FbxObjectTag::AnimationCurveNode => Ok(ClassifiedFbxObject::AnimationCurveNode(
                AnimationCurveNode::try_from(o)?,
            )),
            FbxObjectTag::Unknown => Ok(ClassifiedFbxObject::Unknown(o)),
        }
    }
}

/// Success result of classifying an [`OwnedObject`]: a concrete wrapper or an explicit unknown bucket.
///
/// Use [`ClassifiedFbxObject::inner`] to recover the underlying [`OwnedObject`] and connections.
#[derive(Debug, PartialEq)]
pub enum ClassifiedFbxObject {
    Model(Model),
    MeshGeometry(MeshGeometry),
    LineGeometry(LineGeometry),
    ShapeGeometry(ShapeGeometry),
    Camera(Camera),
    CameraSwitcher(CameraSwitcher),
    Light(Light),
    NullNode(NullNode),
    LimbNode(LimbNode),
    Material(Material),
    Texture(Texture),
    LayeredTexture(LayeredTexture),
    Video(Video),
    Cluster(Cluster),
    Skin(Skin),
    BlendShape(BlendShape),
    BlendShapeChannel(BlendShapeChannel),
    AnimationStack(AnimationStack),
    AnimationLayer(AnimationLayer),
    AnimationCurve(AnimationCurve),
    AnimationCurveNode(AnimationCurveNode),
    /// `Deformer` object with unhandled class name.
    UnknownDeformer(OwnedObject),
    /// `Geometry` object with unhandled class name.
    UnknownGeometry(OwnedObject),
    /// `NodeAttribute` object with unhandled class name.
    UnknownNodeAttribute(OwnedObject),
    /// Any `Objects` row not mapped to a known Assimp DOM class pair.
    Unknown(OwnedObject),
}

impl ClassifiedFbxObject {
    pub fn object_index(&self) -> u64 {
        self.inner().object_index
    }

    pub fn inner(&self) -> &OwnedObject {
        match self {
            ClassifiedFbxObject::Model(x) => x.inner(),
            ClassifiedFbxObject::MeshGeometry(x) => x.inner(),
            ClassifiedFbxObject::LineGeometry(x) => x.inner(),
            ClassifiedFbxObject::ShapeGeometry(x) => x.inner(),
            ClassifiedFbxObject::Camera(x) => x.inner(),
            ClassifiedFbxObject::CameraSwitcher(x) => x.inner(),
            ClassifiedFbxObject::Light(x) => x.inner(),
            ClassifiedFbxObject::NullNode(x) => x.inner(),
            ClassifiedFbxObject::LimbNode(x) => x.inner(),
            ClassifiedFbxObject::Material(x) => x.inner(),
            ClassifiedFbxObject::Texture(x) => x.inner(),
            ClassifiedFbxObject::LayeredTexture(x) => x.inner(),
            ClassifiedFbxObject::Video(x) => x.inner(),
            ClassifiedFbxObject::Cluster(x) => x.inner(),
            ClassifiedFbxObject::Skin(x) => x.inner(),
            ClassifiedFbxObject::BlendShape(x) => x.inner(),
            ClassifiedFbxObject::BlendShapeChannel(x) => x.inner(),
            ClassifiedFbxObject::AnimationStack(x) => x.inner(),
            ClassifiedFbxObject::AnimationLayer(x) => x.inner(),
            ClassifiedFbxObject::AnimationCurve(x) => x.inner(),
            ClassifiedFbxObject::AnimationCurveNode(x) => x.inner(),
            ClassifiedFbxObject::UnknownDeformer(o) => o,
            ClassifiedFbxObject::UnknownGeometry(o) => o,
            ClassifiedFbxObject::UnknownNodeAttribute(o) => o,
            ClassifiedFbxObject::Unknown(o) => o,
        }
    }
}
