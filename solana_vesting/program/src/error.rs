use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error)]
pub enum VestingError {
    #[error("invlaid instruction")]
    InvalidInstruction,
    #[error("Unauthorised")]
    Unauthorised,
    #[error("invalid accounts data")]
    InvalidAccountData,
    #[error("Invalid seed or pda")]
    InvalidSeeds,
    #[error("Nothing Claimable")]
    NothingClaimable,
    #[error("Already claimed full")]
    AlreadyFullyClaimed,
    #[error("Escrow not empty")]
    EscrowNotEmpty,
    #[error("Account not rent exempt")]
    NotRentExempt,
    #[error("Math Overflow")]
    MathOverFlow,
}

impl From<VestingError> for ProgramError {
    fn from(value: VestingError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
