use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    error::LendError,
    state::{Obligations, Reserve},
};
use borsh::{BorshDeserialize, BorshSerialize};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let market_account = next_account_info(account_iter)?;
    let borrow_reserve_account = next_account_info(account_iter)?;
    let obligation_account = next_account_info(account_iter)?;
    let borrow_vault_account = next_account_info(account_iter)?;
    let user_account = next_account_info(account_iter)?;
    let user_ata_account = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;

    if !user_account.is_signer {
        return Err(LendError::Unauthorized.into());
    }
    if market_account.owner != program_id
        || borrow_reserve_account.owner != program_id
        || obligation_account.owner != program_id
    {
        return Err(LendError::MismatchedAccounts.into());
    }

    let market = Market::try_from_slice(&market_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let mut reserve = Reserve::try_from_slice(&borrow_reserve_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let mut obligation = Obligations::try_from_slice(&obligation_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // transfer from the user -> vault
    let transfer_instruction = spl_token::instruction::transfer(
        token_program.key,
        user_ata_account.key,
        &reserve.vault,
        user_account.key,
        &[],
        amount,
    )?;

    invoke(
        &transfer_instruction,
        &[
            token_program.clone(),
            user_account.clone(),
            user_ata_account.clone(),
            borrow_vault_account.clone(),
        ],
    );

    // decrease borrowed scaled
    let scaled_delta = (amount as u128).saturating_mul(crate::math::RAY) / reserve.borrow_index;
    crate::state::Obligations::upsert_position(
        &mut obligation.borrows,
        *borrow_reserve_account.key,
        -(scaled_delta as i128),
        false,
    );
    reserve.total_scaled_borrows = reserve.total_scaled_borrows.saturating_sub(scaled_delta);

    // persist as always. W
    borrow_reserve_account
        .try_borrow_mut_data()?
        .copy_from_slice(&borsh::to_vec(&reserve).unwrap());
    obligation_account
        .try_borrow_mut_data()?
        .copy_from_slice(&borsh::to_vec(&obligation).unwrap());

    Ok(())
}
