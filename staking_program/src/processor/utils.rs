use crate::error::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn load_account<T: BorshDeserialize>(account: &AccountInfo) -> Result<T, ProgramError> {
    T::try_from_slice(&account.data.borrow()).map_err(|_| ProgramError::InvalidAccountData)
}

pub fn save_account<T: BorshSerialize>(
    account: &AccountInfo,
    data: &T,
) -> Result<(), ProgramError> {
    data.serialize(&mut &mut account.data.borrow_mut()[..])
        .map_err(|_| ProgramError::InvalidAccountData)
}

// maths util function
pub fn checked_add(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_add(b).ok_or(StakeError::MathOverFlow.into())
}

pub fn checked_sub(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_sub(b).ok_or(StakeError::MathOverFlow.into())
}

pub fn checked_mul(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_mul(b).ok_or(StakeError::MathOverFlow.into())
}

pub fn checked_mul_u64_i64(a: u64, b: i64) -> Result<u64, ProgramError> {
    if b < 0 {
        return Err(StakeError::MathOverFlow.into());
    }
    let b_u = b as u64;
    a.checked_mul(b_u).ok_or(StakeError::MathOverFlow.into())
}
