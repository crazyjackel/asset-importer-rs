use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Improve cache locality
#[derive(Default)]
pub struct ImproveCacheLocality;

impl AiPostProcess for ImproveCacheLocality {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::ImproveCacheLocality)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement cache locality improvement
        Ok(())
    }
}
