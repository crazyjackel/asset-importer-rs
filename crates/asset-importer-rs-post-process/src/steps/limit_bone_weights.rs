use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Limit bone weights
#[derive(Default)]
pub struct LimitBoneWeights;

impl AiPostProcess for LimitBoneWeights {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::LimitBoneWeights)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement bone weight limiting
        Ok(())
    }
}
