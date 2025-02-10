use super::{quaternion::AiQuaternion, vector::AiVector3D};

/// Interpolation Method Animation Keys from previous
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AiAnimInterpolation {
    Step,
    #[default]
    Linear,
    SphericalLinear,
    CubicSpline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiVectorKey {
    pub time: f64,
    pub value: AiVector3D,
    pub interpolation: AiAnimInterpolation,
}

impl AiVectorKey{
    pub fn new(time: f64, value: AiVector3D, interpolation: AiAnimInterpolation) -> Self{
        Self { time, value, interpolation }
    }
}

impl Default for AiVectorKey {
    fn default() -> Self {
        Self {
            time: 0f64,
            value: Default::default(),
            interpolation: AiAnimInterpolation::Linear,
        }
    }
}

impl PartialOrd for AiVectorKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl AiVectorKey {
    pub fn val_eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    pub fn val_ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiQuatKey {
    pub time: f64,
    pub value: AiQuaternion,
    pub interpolation: AiAnimInterpolation,
}

impl AiQuatKey{
    pub fn new(time: f64, value: AiQuaternion, interpolation: AiAnimInterpolation) -> Self{
        Self { time, value, interpolation }
    }
}

impl Default for AiQuatKey {
    fn default() -> Self {
        Self {
            time: 0f64,
            value: Default::default(),
            interpolation: AiAnimInterpolation::Linear,
        }
    }
}

impl PartialOrd for AiQuatKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}
impl AiQuatKey {
    pub fn val_eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    pub fn val_ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiMeshKey {
    time: f64,
    value: usize,
}

impl Default for AiMeshKey {
    fn default() -> Self {
        Self {
            time: 0f64,
            value: Default::default(),
        }
    }
}

impl PartialOrd for AiMeshKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl AiMeshKey {
    pub fn val_eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    pub fn val_ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiMeshMorphKey {
    pub time: f64,
    pub values: Vec<u32>,
    pub weights: Vec<f64>,
}

impl Default for AiMeshMorphKey {
    fn default() -> Self {
        Self {
            time: 0f64,
            values: Default::default(),
            weights: Default::default(),
        }
    }
}

impl PartialOrd for AiMeshMorphKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AiAnimBehavior {
    #[default]
    Default,
    Constant,
    Linear,
    Repeat,
}

#[derive(Debug, PartialEq, Default)]
pub struct AiNodeAnim {
    pub node_name: String,
    pub position_keys: Vec<AiVectorKey>,
    pub rotation_keys: Vec<AiQuatKey>,
    pub scaling_keys: Vec<AiVectorKey>,
    pub pre_state: AiAnimBehavior,
    pub post_state: AiAnimBehavior,
}

#[derive(Debug, PartialEq, Default)]
pub struct AiMeshAnim {
    name: String,
    keys: Vec<AiMeshKey>,
}

#[derive(Debug, PartialEq, Default)]
pub struct AiMeshMorphAnim {
    pub name: String,
    pub keys: Vec<AiMeshMorphKey>,
}

#[derive(Debug, PartialEq)]
pub struct AiAnimation {
    pub name: String,
    pub duration: f64,
    pub ticks_per_second: f64,
    pub channels: Vec<AiNodeAnim>,
    pub mesh_channels: Vec<AiMeshAnim>,
    pub morph_channels: Vec<AiMeshMorphAnim>,
}

impl Default for AiAnimation {
    fn default() -> Self {
        Self {
            name: Default::default(),
            duration: -1.0,
            ticks_per_second: 0.0,
            channels: Default::default(),
            mesh_channels: Default::default(),
            morph_channels: Default::default(),
        }
    }
}
