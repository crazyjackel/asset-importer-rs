use asset_importer_rs_scene::AiScene;
use enumflags2::{BitFlags, bitflags};

/// Trait for implementing a post-processing step on an [`AiScene`].
///
/// This trait allows you to define custom post-processing steps that can be
/// conditionally applied to a scene after import, based on a set of enabled
/// [`AiPostProcessSteps`].
///
/// # Example
///
/// ```rust
/// use asset_importer_rs_scene::AiScene;
/// use asset_importer_rs_core::post_process::{AiPostProcess, AiPostProcessSteps};
/// use enumflags2::BitFlags;
///
/// struct MyPostProcess;
///
/// impl AiPostProcess for MyPostProcess {
///     type Error = ();
///
///     fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
///         steps.contains(AiPostProcessSteps::Triangulate)
///     }
///
///     fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
///         // Perform post-processing on the scene here
///         Ok(())
///     }
/// }
/// ```
///
/// # Methods
///
/// - [`prepare`]: Prepares this post-process step and returns whether it will be applied.
/// - [`process`]: Applies the post-process step to the given scene.
pub trait AiPostProcess {
    /// The error type returned by the post-process step.
    type Error;

    /// Prepares this post-process step by updating its internal settings based on the enabled steps,
    /// and returns whether or not this step will be applied.
    ///
    /// # Arguments
    ///
    /// * `steps` - The set of enabled post-process steps.
    ///
    /// # Returns
    ///
    /// * `true` if this step is active and will be applied, `false` otherwise.
    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool;

    /// Applies the post-process step to the given scene.
    ///
    /// # Arguments
    ///
    /// * `scene` - Mutable reference to the [`AiScene`] to process.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the process succeeds, or an error of type [`Self::Error`] if it fails.
    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error>;
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AiPostProcessSteps {
    CalcTangentSpaces = 0x1,
    JoinIdenticalVertices = 0x2,
    MakeLeftHanded = 0x4,
    Triangulate = 0x8,
    RemoveComponent = 0x10,
    GenNormals = 0x20,
    GenSmoothNormals = 0x40,
    SplitLargeMeshes = 0x80,
    PreTransformVertices = 0x100,
    LimitBoneWeights = 0x200,
    ValidateDataStructure = 0x400,
    ImproveCacheLocality = 0x800,
    RemoveRedundantMaterials = 0x1000,
    FixInfacingNormals = 0x2000,
    PopulateArmatureData = 0x4000,
    SortByPType = 0x8000,
    FindDegenerates = 0x10000,
    FindInvalidData = 0x20000,
    GenUVCoords = 0x40000,
    TransformUVCoords = 0x80000,
    FindInstances = 0x100000,
    OptimizeMeshes = 0x200000,
    OptimizeGraph = 0x400000,
    FlipUVs = 0x800000,
    FlipWindingOrder = 0x1000000,
    SplitByBoneCount = 0x2000000,
    Debone = 0x4000000,
    GlobalScale = 0x8000000,
    EmbedTextures = 0x10000000,
    ForceGenNormals = 0x20000000,
    DropNormals = 0x40000000,
    GenBoundingBoxes = 0x80000000,
}
