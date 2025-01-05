const EPSILON: f32 = 10e-3f32;

#[derive(Debug, PartialEq, Clone)]
pub struct AiColor4D {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl AiColor4D {
    fn is_black(&self) -> bool {
        f32::abs(self.r) < EPSILON && f32::abs(self.g) < EPSILON && f32::abs(self.b) < EPSILON
    }
}
