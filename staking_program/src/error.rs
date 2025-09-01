// custom errors
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum StakeError {
    #[error("invalid instruction")]
    InValidInstruction,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Math Overflow")]
    MathOverFlow,
}

impl From<StakeError> for ProgramError {
    fn from(value: StakeError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
