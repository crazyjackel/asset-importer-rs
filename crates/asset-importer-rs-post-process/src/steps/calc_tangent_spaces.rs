use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Calculate tangent spaces for meshes
#[derive(Default)]
pub struct CalcTangentSpaces;

impl AiPostProcess for CalcTangentSpaces {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::CalcTangentSpaces)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement tangent space calculation
        Ok(())
    }
}
