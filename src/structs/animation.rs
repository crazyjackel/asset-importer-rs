use super::{quaternion::AiQuaternion, vector::AiVector3D};

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum AiAnimInterpolation {
    Step,
    Linear,
    SphericalLinear,
    CubicSpline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiVectorKey {
    time: f64,
    value: AiVector3D,
    interpolation: AiAnimInterpolation,
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
    time: f64,
    value: AiQuaternion,
    interpolation: AiAnimInterpolation,
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
    time: f64,
    values: Vec<u32>,
    weights: Vec<f64>,
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
#[derive(Debug, PartialEq, Clone)]
pub enum AiAnimBehavior {
    Default,
    Constant,
    Linear,
    Repeat,
}

impl Default for AiAnimBehavior {
    fn default() -> Self {
        AiAnimBehavior::Default
    }
}

#[derive(Debug, PartialEq)]
struct AiNodeAnim {
    node_name: String,
    position_keys: Vec<AiVectorKey>,
    rotation_keys: Vec<AiQuatKey>,
    scaling_keys: Vec<AiVectorKey>,
    pre_state: AiAnimBehavior,
    post_state: AiAnimBehavior,
}

impl Default for AiNodeAnim {
    fn default() -> Self {
        Self {
            node_name: Default::default(),
            position_keys: Default::default(),
            rotation_keys: Default::default(),
            scaling_keys: Default::default(),
            pre_state: Default::default(),
            post_state: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct AiMeshAnim {
    name: String,
    keys: Vec<AiMeshKey>,
}

impl Default for AiMeshAnim {
    fn default() -> Self {
        Self {
            name: Default::default(),
            keys: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct AiMeshMorphAnim {
    name: String,
    keys: Vec<AiMeshMorphKey>,
}

impl Default for AiMeshMorphAnim {
    fn default() -> Self {
        Self {
            name: Default::default(),
            keys: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AiAnimation {
    name: String,
    duration: f64,
    ticks_per_second: f64,
    channels: Vec<AiNodeAnim>,
    mesh_channels: Vec<AiMeshAnim>,
    morph_channels: Vec<AiMeshMorphAnim>,
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
