use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::BitFlags;

/// Embed textures
#[derive(Default)]
pub struct EmbedTextures;

impl AiPostProcess for EmbedTextures {
    type Error = String;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::EmbedTextures)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        // TODO: Implement texture embedding
        Ok(())
    }
}
