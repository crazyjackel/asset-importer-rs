use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Make meshes left-handed
#[derive(Default)]
pub struct MakeLeftHanded;

impl AiPostProcess for MakeLeftHanded {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::MakeLeftHanded)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement left-handed conversion
        Ok(())
    }
}
