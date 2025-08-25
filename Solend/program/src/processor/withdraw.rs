use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction;

use crate::{error::*, math::RAY, state::*};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    _max_slippage_bps: u16,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let market_account = next_account_info(account_iter)?;

    let reserve_account = next_account_info(account_iter)?;

    let obligation_account = next_account_info(account_iter)?;

    let vault_account = next_account_info(account_iter)?;

    let user_account = next_account_info(account_iter)?;

    let user_ata_account = next_account_info(account_iter)?;

    let token_program = next_account_info(account_iter)?;

    let _oracle_id = next_account_info(account_iter)?;

    if !user_account.is_signer {
        return Err(LendError::Unauthorized.into());
    }

    if market_account.owner != program_id
        || reserve_account.owner != program_id
        || obligation_account.owner != program_id
    {
        return Err(LendError::MismatchedAccounts.into());
    };

    let market = Market::try_from_slice(&market_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if market.paused {
        return Err(LendError::Paused.into());
    }

    let reserve = Reserve::try_from_slice(&reserve_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let obligation = Obligations::try_from_slice(&obligation_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // real amount to scaled amount
    let scaled_delta = (amount as u128).saturating_mul(crate::math::RAY) / reserve.liquidity_index;

    // decrease user collateral position
    crate::state::Obligation::upsert_position(
        &mut obligation.collaterals,
        *reserve_account.key,
        -(scaled_delta as i128),
        true,
    );

    // transfer from vault ->  user

    let instruction = instruction::transfer(
        token_program.key,
        &reserve.vault,
        user_ata_account.key,
        &crate::id(),
        &[],
        amount,
    );

    invoke(
        &instruction,
        &[
            vault_account.clone(),
            user_ata_account.clone(),
            token_program.clone(),
        ],
    )?;

    // reduce totals
    reserve.total_scaled_deposits = reserve.total_scaled_deposits.saturating_sub(scaled_delta);

    // persist

    reserve_account
        .try_borrow_mut_data()?
        .copy_from_slice(&borsh::to_vec(&reserve).unwrap());

    obligation_ai
        .try_borrow_mut_data()?
        .copy_from_slice(&borsh::to_vec(&obligation).unwrap());

    Ok(())
}
