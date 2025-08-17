use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Find degenerate triangles
#[derive(Default)]
pub struct FindDegenerates;

impl AiPostProcess for FindDegenerates {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::FindDegenerates)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement degenerate triangle detection
        Ok(())
    }
}
