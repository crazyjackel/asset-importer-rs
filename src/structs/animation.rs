use super::vector::AiVector3D;


#[repr(u8)]
#[derive(Debug,PartialEq)]
pub enum AiAnimInterpolation{
    Step,
    Linear,
    SphericalLinear,
    CubicSpline
}

#[derive(Debug, PartialEq)]
pub struct AiVectorKey{
    time: f64,
    value: AiVector3D,
    interpolation: AiAnimInterpolation
}

impl Default for AiVectorKey{
    fn default() -> Self {
        Self { time: 0f64, value: Default::default(), interpolation: AiAnimInterpolation::Linear }
    }
}

impl PartialOrd for AiVectorKey{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

#[derive(Debug, PartialEq)]
pub struct AiQuatKey{
    time: f64,
    value: AiQuaternion,
    interpolation: AiAnimInterpolation
}

impl Default for AiQuatKey{
    fn default() -> Self {
        Self { time: 0f64, value: Default::default(), interpolation: AiAnimInterpolation::Linear }
    }
}

impl PartialOrd for AiQuatKey{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}


#[derive(Debug, PartialEq)]
pub struct AiAnimation;