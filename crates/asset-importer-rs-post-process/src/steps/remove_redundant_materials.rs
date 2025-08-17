use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Remove redundant materials
#[derive(Default)]
pub struct RemoveRedundantMaterials;

impl AiPostProcess for RemoveRedundantMaterials {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::RemoveRedundantMaterials)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement redundant material removal
        Ok(())
    }
}
