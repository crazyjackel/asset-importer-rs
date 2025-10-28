use asset_importer_rs_core::{AiPostProcess, AiPostProcessSteps};
use asset_importer_rs_scene::AiScene;
use enumflags2::{BitFlags, bitflags};

#[derive(Debug, PartialEq)]
pub enum FlipUVsError {
    UvsNotClippedError(String, usize),
}
impl std::fmt::Display for FlipUVsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlipUVsError::UvsNotClippedError(channel_name, channel_index) => {
                write!(
                    f,
                    "UVs in channel named ({}) and with channel index ({}) do not have all their uv components between 0.0 and 1.0 ",
                    channel_name, channel_index
                )
            }
        }
    }
}
impl std::error::Error for FlipUVsError {}

#[derive(Copy, Clone)]
pub enum UvFlipVariant {
    X = 0x1,
    Y = 0x2,
    Z = 0x4,
}
impl Default for UvFlipVariant {
    fn default() -> Self {
        Self::Y
    }
}

/// Flip UVs
#[derive(Default)]
pub struct FlipUVs {
    first_flip: UvFlipVariant,
    second_flip: Option<UvFlipVariant>,
}

impl AiPostProcess for FlipUVs {
    type Error = FlipUVsError;

    fn prepare(&mut self, steps: BitFlags<AiPostProcessSteps>) -> bool {
        steps.contains(AiPostProcessSteps::FlipUVs)
    }

    fn process(&self, scene: &mut AiScene) -> Result<(), Self::Error> {
        for mesh in scene.meshes.iter_mut() {
            let mut flips = vec![self.first_flip];
            if let Some(second_flip) = self.second_flip {
                flips.push(second_flip)
            }

            let uv_channel_names = &mesh.texture_coordinate_names;
            let mut uv_channels = mesh.texture_coords.iter_mut().enumerate();
            while let (channel_index, Some(uvs)) = uv_channels.next().unwrap() {
                for uv in uvs.iter_mut() {
                    if uv.z != 0.0 && uv.square_length() > 3.0 {
                        return Err(FlipUVsError::UvsNotClippedError(
                            uv_channel_names[channel_index].clone(),
                            channel_index,
                        ));
                    } else if uv.z == 0.0 && uv.square_length() > 2.0 {
                        return Err(FlipUVsError::UvsNotClippedError(
                            uv_channel_names[channel_index].clone(),
                            channel_index,
                        ));
                    }

                    for flip in flips.iter() {
                        match flip {
                            UvFlipVariant::X => uv.x = -uv.x + 1.0f32,
                            UvFlipVariant::Y => uv.y = -uv.y + 1.0f32,
                            UvFlipVariant::Z => uv.z = -uv.z + 1.0f32,
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asset_importer_rs_scene::{
        AI_MAX_NUMBER_OF_TEXTURECOORDS, AiFace, AiMesh, AiPrimitiveType, AiSceneFlag, AiVector2D,
        AiVector3D,
    };
    use enumflags2::BitFlags;

    fn create_test_mesh() -> AiMesh {
        let mut texture_coords_channels= // : [Option<Vec<AiVector3D>>; AI_MAX_NUMBER_OF_TEXTURECOORDS] =
            [const { None }; AI_MAX_NUMBER_OF_TEXTURECOORDS];
        texture_coords_channels[0] = Some(vec![[0.0, 0.0, 0.0].into(), [0.0, 1.0, 0.0].into()]);
        AiMesh {
            texture_coords: texture_coords_channels,
            ..Default::default()
        }
    }

    fn create_test_scene() -> AiScene {
        AiScene {
            meshes: vec![create_test_mesh()],
            flags: BitFlags::empty(), // Not non-verbose
            ..Default::default()
        }
    }

    #[test]
    fn test_uv_flipping() {
        let mut scene = create_test_scene();
        let mut yflipper = FlipUVs::default();

        yflipper.prepare(BitFlags::from(AiPostProcessSteps::FlipUVs));
        let result = yflipper.process(&mut scene);
        assert!(result.is_ok());

        let tex_coords = scene.meshes[0].texture_coords[0].as_ref().unwrap();
        assert_eq!(
            *tex_coords,
            vec![[0.0, 1.0, 0.0].into(), [0.0, 0.0, 0.0].into()]
        )
    }
}
