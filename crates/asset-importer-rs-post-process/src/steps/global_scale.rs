use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Apply global scale
#[derive(Default)]
pub struct GlobalScale;

impl AiPostProcess for GlobalScale {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::GlobalScale)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement global scaling
        Ok(())
    }
}
