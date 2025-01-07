
use super::type_def::EPSILON_F;

#[derive(Debug, PartialEq, Clone)]
pub struct AiColor3D {
    r: f32,
    g: f32,
    b: f32
}


impl Default for AiColor3D{
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default() }
    }
}

impl AiColor3D {
    pub fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON_F && f32::abs(self.g) < EPSILON_F && f32::abs(self.b) < EPSILON_F
    }
    pub fn new(pr: f32, pg: f32, pb: f32) -> Self{
        AiColor3D{r: pr, g: pg, b: pb}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AiColor4D {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}


impl Default for AiColor4D{
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default(), a: Default::default() }
    }
}

impl AiColor4D {
    pub fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON_F && f32::abs(self.g) < EPSILON_F && f32::abs(self.b) < EPSILON_F
    }
    pub fn new(pr: f32, pg: f32, pb: f32, pa: f32) -> Self{
        AiColor4D{r: pr, g: pg, b: pb, a: pa}
    }
}