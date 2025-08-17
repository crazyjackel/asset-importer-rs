use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Force generate normals
#[derive(Default)]
pub struct ForceGenNormals;

impl AiPostProcess for ForceGenNormals {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::ForceGenNormals)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement forced normal generation
        Ok(())
    }
}
