use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Transform UV coordinates
#[derive(Default)]
pub struct TransformUVCoords;

impl AiPostProcess for TransformUVCoords {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::TransformUVCoords)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement UV coordinate transformation
        Ok(())
    }
}
