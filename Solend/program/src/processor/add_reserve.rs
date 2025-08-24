use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::{error::*, instruction::*, math::RAY, state::*};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: ReserveParams,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let market_account = next_account_info(account_iter)?;
    let reserve_account = next_account_info(account_iter)?;
    let vault_account = next_account_info(account_iter)?;
    let mint_account = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let admin_account = next_account_info(account_iter)?;

    if !admin_account.is_signer {
        return Err(LendError::Unauthorized.into());
    };
    if market_account.owner != program_id {
        return Err(LendError::MismatchedAccounts.into());
    };

    if reserve_account.owner != program_id {
        return Err(LendError::MismatchedAccounts.into());
    };

    // quick validation of markett
    {
        let market_data = market_account.try_borrow_data()?;
        let data =
            Market::try_from_slice(&market_data).map_err(|_| ProgramError::InvalidAccountData)?;

        if data.admin != *admin_account.key {
            return Err(LendError::Unauthorized.into());
        };
    }

    // initialize reserve
    let reserve = Reserve {
        market: *market_account.key,
        token_mint: *mint_account.key,
        vault: *vault_account.key,
        decimals: params.decimals,
        base_rate_per_year_bps: params.base_rate_per_year_bps,
        slope1_bps: params.slope1_bps,
        slope2_bps: params.slope2_bps,
        kink_bps: params.kink_bps,
        liquidity_index: RAY,
        last_update: 0,
        total_scaled_deposits: 0,
        total_scaled_borrows: 0,
        ltv_bps: params.ltv_bps,
        liquidation_threshold_bps: params.liquidation_threshold_bps,
        liquidation_bonus_bps: params.liquidation_bonus_bps,
        bump: 0,
        paused: false,
    };

    let mut reserve_data = reserve_account.try_borrow_mut_data()?;
    reserve.serialize(&mut &mut *reserve_account.data[..])?;

    // assuming : vault ata already exists and owned by this program via pda.

    Ok(())
}
