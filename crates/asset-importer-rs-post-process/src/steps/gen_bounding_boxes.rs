use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Generate bounding boxes
#[derive(Default)]
pub struct GenBoundingBoxes;

impl AiPostProcess for GenBoundingBoxes {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::GenBoundingBoxes)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement bounding box generation
        Ok(())
    }
}
