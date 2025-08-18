//! Post-processing module for asset-importer-rs
//!
//! This module provides implementations of various post-processing steps
//! that can be applied to 3D assets after import.

pub mod error;
pub mod helper;
pub mod steps;

// Re-export commonly used items
pub use asset_importer_rs_core::AiPostProcessSteps;
pub use asset_importer_rs_scene::AiScene;
pub use error::AiPostProcessError;

use asset_importer_rs_core::AiPostProcess;
use enumflags2::BitFlags;

// Re-export steps based on features
#[cfg(feature = "calc-tangent-spaces")]
pub use steps::CalcTangentSpaces;
#[cfg(feature = "find-degenerates")]
pub use steps::FindDegenerates;
#[cfg(feature = "find-invalid-data")]
pub use steps::FindInvalidData;
#[cfg(feature = "gen-normals")]
pub use steps::GenNormals;
#[cfg(feature = "gen-smooth-normals")]
pub use steps::GenSmoothNormals;
#[cfg(feature = "join-identical-vertices")]
pub use steps::JoinIdenticalVertices;
#[cfg(feature = "optimize-graph")]
pub use steps::OptimizeGraph;
#[cfg(feature = "optimize-meshes")]
pub use steps::OptimizeMeshes;
#[cfg(feature = "remove-redundant-materials")]
pub use steps::RemoveRedundantMaterials;
#[cfg(feature = "triangulate")]
pub use steps::Triangulate;
#[cfg(feature = "validate-data-structure")]
pub use steps::ValidateDataStructure;

// Additional feature re-exports
#[cfg(feature = "debone")]
pub use steps::Debone;
#[cfg(feature = "drop-normals")]
pub use steps::DropNormals;
#[cfg(feature = "embed-textures")]
pub use steps::EmbedTextures;
#[cfg(feature = "find-instances")]
pub use steps::FindInstances;
#[cfg(feature = "fix-infacing-normals")]
pub use steps::FixInfacingNormals;
#[cfg(feature = "flip-uvs")]
pub use steps::FlipUVs;
#[cfg(feature = "flip-winding-order")]
pub use steps::FlipWindingOrder;
#[cfg(feature = "force-gen-normals")]
pub use steps::ForceGenNormals;
#[cfg(feature = "gen-bounding-boxes")]
pub use steps::GenBoundingBoxes;
#[cfg(feature = "gen-uv-coords")]
pub use steps::GenUVCoords;
#[cfg(feature = "global-scale")]
pub use steps::GlobalScale;
#[cfg(feature = "improve-cache-locality")]
pub use steps::ImproveCacheLocality;
#[cfg(feature = "limit-bone-weights")]
pub use steps::LimitBoneWeights;
#[cfg(feature = "make-left-handed")]
pub use steps::MakeLeftHanded;
#[cfg(feature = "populate-armature-data")]
pub use steps::PopulateArmatureData;
#[cfg(feature = "pre-transform-vertices")]
pub use steps::PreTransformVertices;
#[cfg(feature = "remove-component")]
pub use steps::RemoveComponent;
#[cfg(feature = "sort-by-p-type")]
pub use steps::SortByPType;
#[cfg(feature = "split-by-bone-count")]
pub use steps::SplitByBoneCount;
#[cfg(feature = "split-large-meshes")]
pub use steps::SplitLargeMeshes;
#[cfg(feature = "transform-uv-coords")]
pub use steps::TransformUVCoords;

type AiPostProcessDyn = dyn AiPostProcess<Error = AiPostProcessError>;

pub struct PostProcess {
    processors: Vec<Box<AiPostProcessDyn>>,
}

impl PostProcess {
    pub fn new(processors: Vec<Box<AiPostProcessDyn>>) -> Self {
        Self { processors }
    }

    pub fn process(
        &mut self,
        scene: &mut AiScene,
        steps: BitFlags<AiPostProcessSteps>,
    ) -> Result<(), AiPostProcessError> {
        for processor in self.processors.iter_mut() {
            if processor.prepare(steps) {
                processor.process(scene)?;
            }
        }
        Ok(())
    }
}

pub struct AiPostProcesserWrapper<T: AiPostProcess>
where
    T::Error: Into<AiPostProcessError>,
{
    inner: T,
}

impl<T: AiPostProcess> AiPostProcesserWrapper<T>
where
    T::Error: Into<AiPostProcessError>,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: AiPostProcess> AiPostProcess for AiPostProcesserWrapper<T>
where
    T::Error: Into<AiPostProcessError>,
{
    type Error = AiPostProcessError;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        self.inner.prepare(steps)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        self.inner.process(scene).map_err(Into::into)
    }
}
pub struct AiPostProcesser;

impl AiPostProcesser {
    pub fn post_process() -> PostProcess {
        PostProcess::new(vec![
            #[cfg(feature = "calc-tangent-spaces")]
            Box::new(AiPostProcesserWrapper::new(CalcTangentSpaces)),
            #[cfg(feature = "find-degenerates")]
            Box::new(AiPostProcesserWrapper::new(FindDegenerates)),
            #[cfg(feature = "find-invalid-data")]
            Box::new(AiPostProcesserWrapper::new(FindInvalidData)),
            #[cfg(feature = "gen-normals")]
            Box::new(AiPostProcesserWrapper::new(GenNormals::default())),
            #[cfg(feature = "gen-smooth-normals")]
            Box::new(AiPostProcesserWrapper::new(GenSmoothNormals::default())),
            #[cfg(feature = "join-identical-vertices")]
            Box::new(AiPostProcesserWrapper::new(JoinIdenticalVertices)),
            #[cfg(feature = "optimize-graph")]
            Box::new(AiPostProcesserWrapper::new(OptimizeGraph)),
            #[cfg(feature = "optimize-meshes")]
            Box::new(AiPostProcesserWrapper::new(OptimizeMeshes)),
            #[cfg(feature = "remove-redundant-materials")]
            Box::new(AiPostProcesserWrapper::new(RemoveRedundantMaterials)),
            #[cfg(feature = "triangulate")]
            Box::new(AiPostProcesserWrapper::new(Triangulate)),
            #[cfg(feature = "validate-data-structure")]
            Box::new(AiPostProcesserWrapper::new(ValidateDataStructure)),
            #[cfg(feature = "debone")]
            Box::new(AiPostProcesserWrapper::new(Debone)),
            #[cfg(feature = "drop-normals")]
            Box::new(AiPostProcesserWrapper::new(DropNormals)),
            #[cfg(feature = "embed-textures")]
            Box::new(AiPostProcesserWrapper::new(EmbedTextures)),
            #[cfg(feature = "find-instances")]
            Box::new(AiPostProcesserWrapper::new(FindInstances)),
            #[cfg(feature = "fix-infacing-normals")]
            Box::new(AiPostProcesserWrapper::new(FixInfacingNormals)),
            #[cfg(feature = "flip-uvs")]
            Box::new(AiPostProcesserWrapper::new(FlipUVs)),
            #[cfg(feature = "flip-winding-order")]
            Box::new(AiPostProcesserWrapper::new(FlipWindingOrder)),
            #[cfg(feature = "force-gen-normals")]
            Box::new(AiPostProcesserWrapper::new(ForceGenNormals)),
            #[cfg(feature = "gen-bounding-boxes")]
            Box::new(AiPostProcesserWrapper::new(GenBoundingBoxes)),
            #[cfg(feature = "gen-uv-coords")]
            Box::new(AiPostProcesserWrapper::new(GenUVCoords)),
            #[cfg(feature = "global-scale")]
            Box::new(AiPostProcesserWrapper::new(GlobalScale)),
            #[cfg(feature = "improve-cache-locality")]
            Box::new(AiPostProcesserWrapper::new(ImproveCacheLocality)),
            #[cfg(feature = "limit-bone-weights")]
            Box::new(AiPostProcesserWrapper::new(LimitBoneWeights)),
            #[cfg(feature = "make-left-handed")]
            Box::new(AiPostProcesserWrapper::new(MakeLeftHanded)),
            #[cfg(feature = "populate-armature-data")]
            Box::new(AiPostProcesserWrapper::new(PopulateArmatureData)),
            #[cfg(feature = "pre-transform-vertices")]
            Box::new(AiPostProcesserWrapper::new(PreTransformVertices)),
            #[cfg(feature = "remove-component")]
            Box::new(AiPostProcesserWrapper::new(RemoveComponent)),
            #[cfg(feature = "sort-by-p-type")]
            Box::new(AiPostProcesserWrapper::new(SortByPType)),
            #[cfg(feature = "split-by-bone-count")]
            Box::new(AiPostProcesserWrapper::new(SplitByBoneCount)),
            #[cfg(feature = "split-large-meshes")]
            Box::new(AiPostProcesserWrapper::new(SplitLargeMeshes)),
            #[cfg(feature = "transform-uv-coords")]
            Box::new(AiPostProcesserWrapper::new(TransformUVCoords)),
        ])
    }
}
