use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Join identical vertices in meshes
#[derive(Default)]
pub struct JoinIdenticalVertices;

impl AiPostProcess for JoinIdenticalVertices {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::JoinIdenticalVertices)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement vertex joining
        Ok(())
    }
}
