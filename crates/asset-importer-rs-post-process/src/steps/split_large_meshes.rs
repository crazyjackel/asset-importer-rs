use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Split large meshes
#[derive(Default)]
pub struct SplitLargeMeshes;

impl AiPostProcess for SplitLargeMeshes {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::SplitLargeMeshes)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement large mesh splitting
        Ok(())
    }
}
