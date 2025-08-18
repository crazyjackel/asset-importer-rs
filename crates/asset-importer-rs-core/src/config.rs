use asset_importer_rs_scene::AiReal;

/// Configuration key for checking identity matrix epsilon.
///
/// This constant defines the epsilon value used when checking if a matrix
/// is close enough to an identity matrix to be considered equal.
pub const AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON: &str = "CHECK_IDENTITY_MATRIX_EPSILON";

/// Default epsilon value for identity matrix checking.
///
/// The default tolerance used when comparing matrices to the identity matrix.
/// A value of 10e-3 means matrices with differences smaller than 0.01 are considered equal.
pub const AI_CONFIG_CHECK_IDENTITY_MATRIX_EPSILON_DEFAULT: AiReal = 10e-3f32 as AiReal;

/// Configuration key for using GLTF PBR specular-glossiness workflow.
///
/// When enabled, the importer will use the specular-glossiness PBR workflow
/// instead of the metallic-roughness workflow for GLTF files.
pub const AI_CONFIG_USE_GLTF_PBR_SPECULAR_GLOSSINESS: &str = "USE_GLTF_PBR_SPECULAR_GLOSSINESS";

/// Configuration key for GLTF2 node transformation representation.
///
/// Controls how node transformations are represented in GLTF2 files,
/// specifically whether to use TRS (Translation, Rotation, Scale) format.
pub const GLTF2_NODE_IN_TRS: &str = "GLTF2_NODE_IN_TRS";

/// Configuration key for unlimited skinning bones per vertex in GLTF export.
///
/// When enabled, removes the limit on the number of bones that can influence
/// a single vertex during GLTF export, allowing for more complex skeletal animations.
pub const AI_CONFIG_EXPORT_GLTF_UNLIMITED_SKINNING_BONES_PER_VERTEX: &str =
    "AI_CONFIG_EXPORT_GLTF_UNLIMITED_SKINNING_BONES_PER_VERTEX";

/// Configuration key for GLTF2 target normal export.
///
/// Controls the export of normal targets in GLTF2 files, which are used
/// for morph target animations and normal map blending.
pub const GLTF2_TARGET_NORMAL_EXP: &str = "GLTF2_TARGET_NORMAL_EXP";
