use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum LendError {
    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Unauthorize")]
    Unauthorized,

    #[error("Math Overflow")]
    MathOverflow,

    #[error("Health factor too low")]
    HealthFactorTooLow,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    #[error("Paused")]
    Paused,

    #[error("Mismatched accounts")]
    MismatchedAccounts,

    #[error("Oracle invalid")]
    OracleInvalid,

    #[error("Not implemented")]
    NotImplemented,
}

impl From<LendError> for ProgramError {
    fn from(value: LendError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
