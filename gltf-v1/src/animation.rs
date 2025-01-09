

use super::Document;



/// A keyframe animation.
#[derive(Clone, Debug)]
pub struct Animation<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: String,

    /// The corresponding JSON struct.
    json: &'a gltf_v1_json::Animation
}