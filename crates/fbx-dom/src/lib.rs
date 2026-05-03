//! FBX document model and typed object layer for asset import.
//!
//! ## Pipeline
//!
//! 1. **Load** — [`Document::from_parser`] (ASCII via [`fbxscii`]) or [`Document::from_binary_reader`]
//!    (binary via [`fbxcel`]). Both fill the same [`Document`]: header, definitions/templates,
//!    per-object element subtrees in the internal arena, and **connection** maps keyed by FBX object
//!    id (`u64`).
//! 2. **Borrowed access** — [`Object`] wraps a [`LazyObject`] + template + [`Document`] for
//!    [`Object::properties`], [`Object::attributes`], and connection helpers (`OO` / `OP` / `PP`).
//! 3. **Owned row** — [`OwnedObject`] copies properties, subtree [`fbxscii::ElementAttribute`] map,
//!    and outgoing connection lists for use without holding a [`Document`].
//! 4. **Classification** — [`objects::ClassifiedFbxObject::try_from`] dispatches
//!    [`OwnedObject`] by `type_name` / `class_name` (Assimp-style) into mesh, material, animation, etc.
//! 5. **Aggregate** — [`OwnedDocument::from`] walks all objects, classifies, and fills typed `Vec`s
//!    plus [`OwnedDocument::unknown_objects`] for rows that fail narrowing.
//!
//! Connection semantics are documented on [`crate::document::ObjectPropertyConnection`] and
//! [`Object`] accessor methods.

mod any_loader;
mod document;
mod global;
mod loader;
mod object;
pub mod objects;
mod owned_document;

pub use document::Document;
pub use document::DocumentParseError;
pub use document::ImportSettings;
pub use document::LazyObject;
pub use document::ObjectPropertyConnection;
pub use document::Property;
pub use document::PropertyDetails;
pub use document::PropertyParseError;
pub use document::Template;

pub use object::Object;
pub use object::ObjectError;
pub use object::Objects;
pub use object::OwnedObject;
pub use owned_document::OwnedDocument;

pub use global::FrameRate;
pub use global::GlobalSettings;

pub use objects::{
    AnimationCurve, AnimationCurveNode, AnimationLayer, AnimationStack, BlendShape,
    BlendShapeChannel, Camera, CameraSwitcher, ClassifiedFbxObject, Cluster, FbxTryFromReason,
    FbxTypeMismatch, LayeredTexture, Light, LightDecay, LightType, LimbNode, LineGeometry,
    Material, MeshGeometry, Model, ModelGeometryRef, ModelRotationOrder, ModelTransformInheritance,
    NodeAttributeRef, NullNode, ShapeGeometry, Skin, Texture, Video,
};
