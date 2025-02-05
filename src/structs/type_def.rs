use std::path::Display;

#[cfg(feature = "use_double_precision")]
pub mod base_types {
    pub type AiReal = f64;
    pub type AiInt = i64;
    pub type AiUint = u64;

    pub const AiMathPI : AiReal = 3.14159265358979323846264338327964;
    pub const AIMathTwoPI : AiReal = AiMathPI * 2.0;
}
#[cfg(not(feature = "use_double_precision"))]
pub mod base_types {
    pub type AiReal = f32;
    pub type AiInt = i32;
    pub type AiUint = u32;

    pub const AiMathPI : AiReal = 3.141592653589793238462643383279f32;
    pub const AIMathTwoPI : AiReal = AiMathPI * 2.0;
}

pub const EPSILON_F: f32 = 10e-3f32;
pub const AiMathPI_F : f32 = 3.141592653589793238462643383279f32;
pub const AIMathTwoPI_F : f32 = AiMathPI_F * 2.0;