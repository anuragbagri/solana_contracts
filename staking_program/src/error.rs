// custom errors
use thiserror::Error;
use solana_program::program_error::ProgramError;

pub enum StakeError {
    #[error("invalid instruction")]
    InValidInstruction,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Math Overflow")]
    MathOverFlow,
};

impl From<StakeError> for ProgramError {
    fn from(value: StakeError) -> Self {
        ProgramError::Custom(value as u32);
    }
}