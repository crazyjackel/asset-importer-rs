#[cfg(feature = "double_precision")]
pub mod base_types {
    pub type AiReal = f64;
    pub type AiInt = i64;
    pub type AiUint = u64;

    pub const AI_MATH_PI: AiReal = std::f64::consts::PI;
    pub const AI_MATH_TWO_PI: AiReal = AI_MATH_PI * 2.0;

    pub const fn radians_to_degrees(radians: AiReal) -> AiReal {
        radians * 57.295_78_f64
    }

    pub const fn degrees_to_radians(degrees: AiReal) -> AiReal {
        degrees * 0.017_453_292_f64
    }
}
#[cfg(not(feature = "double_precision"))]
pub mod base_types {

    pub type AiReal = f32;
    pub type AiInt = i32;
    pub type AiUint = u32;

    pub const AI_MATH_PI: AiReal = std::f32::consts::PI;
    pub const AI_MATH_TWO_PI: AiReal = AI_MATH_PI * 2.0;

    pub const fn radians_to_degrees(radians: AiReal) -> AiReal {
        radians * 57.295_78_f32
    }

    pub const fn degrees_to_radians(degrees: AiReal) -> AiReal {
        degrees * 0.017_453_292_f32
    }
}

pub const EPSILON_F: f32 = 10e-3f32;
pub const AI_MATH_PI_F: f32 = std::f32::consts::PI;
pub const AI_MATH_TWO_PI_F: f32 = AI_MATH_PI_F * 2.0;
