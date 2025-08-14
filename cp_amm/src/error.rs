use solana_program::program_error::ProgramError;
use thiserror::Error;
#[repr(u32)]
#[derive(Debug, Error)]
pub enum AmmErr {
    #[error("pool already initialized")]
    AlreadyInitialized = 0,

    #[error("pool not initialized")]
    Uninitialized,

    #[error("in-valid vault owner")]
    InvalidVaultOwner,

    #[error("mint authority is not valid")]
    InvalidMintAuthority,

    #[error("math overflow or problem  in  constant product algorithm ")]
    MathOverFlow,

    #[error("mint size not sufficient")]
    TooFewLpMinted,

    #[error("slippage exceeded for the pool")]
    SlippageExceeded,
}

impl From<AmmErr> for ProgramError {
    fn from(value: AmmErr) -> Self {
        ProgramError::Custom(value as u32)
    }
}
