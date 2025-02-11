mod animation;

/// glTF JSON wrapper.
#[derive(Clone, Debug)]
pub struct Document(gltf_v1_json::GLTF);
