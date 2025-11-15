use std::{error::Error, fmt::Display};

#[cfg(feature = "flip-uvs")]
use crate::steps::flip_uvs::FlipUVsError;
#[cfg(feature = "gen-normals")]
use crate::steps::gen_normals::GenNormalsError;
#[cfg(feature = "gen-smooth-normals")]
use crate::steps::gen_smooth_normals::GenSmoothNormalsError;

#[non_exhaustive]
#[derive(Debug)]
pub enum AiPostProcessError {
    #[cfg(feature = "gen-normals")]
    GenNormalsError(GenNormalsError),
    #[cfg(feature = "gen-smooth-normals")]
    GenSmoothNormalsError(GenSmoothNormalsError),
    #[cfg(feature = "flip-uvs")]
    FlipUVsError(FlipUVsError),

    PostProcessError(String),
}

impl Display for AiPostProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiPostProcessError::PostProcessError(error) => write!(f, "PostProcessError: {}", error),
            #[cfg(feature = "gen-normals")]
            AiPostProcessError::GenNormalsError(error) => write!(f, "GenNormalsError: {}", error),
            #[cfg(feature = "gen-smooth-normals")]
            AiPostProcessError::GenSmoothNormalsError(error) => {
                write!(f, "GenSmoothNormalsError: {}", error)
            }
            #[cfg(feature = "flip-uvs")]
            AiPostProcessError::FlipUVsError(error) => {
                write!(f, "FlipUVsError: {}", error)
            }
        }
    }
}
impl Error for AiPostProcessError {}

impl From<String> for AiPostProcessError {
    fn from(error: String) -> Self {
        AiPostProcessError::PostProcessError(error)
    }
}

#[cfg(feature = "flip-uvs")]
impl From<FlipUVsError> for AiPostProcessError {
    fn from(error: FlipUVsError) -> Self {
        AiPostProcessError::FlipUVsError(error)
    }
}

#[cfg(feature = "gen-normals")]
impl From<GenNormalsError> for AiPostProcessError {
    fn from(error: GenNormalsError) -> Self {
        AiPostProcessError::PostProcessError(error.to_string())
    }
}

#[cfg(feature = "gen-smooth-normals")]
impl From<GenSmoothNormalsError> for AiPostProcessError {
    fn from(error: GenSmoothNormalsError) -> Self {
        AiPostProcessError::PostProcessError(error.to_string())
    }
}
