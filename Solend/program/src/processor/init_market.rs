use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent},
};

use crate::{error::*, state::*};
use borsh::BorshSerialize;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    oracle_program_id: Pubkey,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let market = next_account_info(account_iter)?;
    let admin = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    if !admin.is_signer {
        return Err(LendError::Unauthorized.into());
    }

    if market.owner != program_id {
        return Err(LendError::MismatchedAccounts.into());
    }

    // initiailzie market data
    let mut market_data = market.try_borrow_mut_data()?; // 2 cases == if empty , write all along , if already written update with new params

    let market_init_data = Market {
        admin: *admin.key,
        oracle_program_id,
        bump: 0,
        paused: false,
    };

    market_init_data.serialize(&mut *market_data)?;

    // rent check for market account
    let rent = Rent::get()?;
    if !rent.is_exempt(market.lamports(), market.data_len()) {
        return Err(LendError::NotRentExempt.into());
    };

    Ok(())
}
