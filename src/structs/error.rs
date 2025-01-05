use core::error;
use std::fmt::Display;


#[derive(Debug,Clone)]
pub struct AiFailure;

impl Display for AiFailure{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to run Assimp Function")
    }
}

impl error::Error for AiFailure {}

#[derive(Debug,Clone)]
pub struct AiOutOfMemory;

impl Display for AiOutOfMemory{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assimp Function does not have enough Memory to Execute")
    }
}

impl error::Error for AiOutOfMemory {}

#[derive(Debug, Clone)]
pub enum AiReturnError{
    Failure(AiFailure),
    OutOfMemory(AiOutOfMemory)
}

impl Display for AiReturnError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            AiReturnError::Failure(ai_failure) => write!(f, "{}", ai_failure),
            AiReturnError::OutOfMemory(ai_out_of_memory) => write!(f, "{}", ai_out_of_memory),
        }
    }
}

impl error::Error for AiReturnError {}