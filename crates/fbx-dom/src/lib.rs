mod any_loader;
mod document;
mod global;
mod loader;
mod object;
pub mod objects;

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

pub use global::FrameRate;
pub use global::GlobalSettings;

pub use objects::{
    AnimationCurve, AnimationCurveNode, AnimationLayer, AnimationStack, BlendShape,
    BlendShapeChannel, Camera, CameraSwitcher, ClassifiedFbxObject, Cluster, FbxTryFromReason,
    FbxTypeMismatch, LayeredTexture, Light, LimbNode, LineGeometry, Material, MeshGeometry, Model,
    NullNode, ShapeGeometry, Skin, Texture, Video,
};
