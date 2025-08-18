use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Fix infacing normals
#[derive(Default)]
pub struct FixInfacingNormals;

impl AiPostProcess for FixInfacingNormals {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::FixInfacingNormals)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement infacing normal fixing
        Ok(())
    }
}
