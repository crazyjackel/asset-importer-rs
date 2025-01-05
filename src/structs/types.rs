use std::path::Display;

#[cfg(feature = "use_double_precision")]
pub mod base_types {
    pub type AiReal = f64;
    pub type AiInt = i64;
    pub type AiUint = u64;
}
#[cfg(not(feature = "use_double_precision"))]
pub mod base_types {
    pub type AiReal = f32;
    pub type AiInt = i32;
    pub type AiUint = u32;
}

