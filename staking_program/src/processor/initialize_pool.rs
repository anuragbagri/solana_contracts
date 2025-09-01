use crate::{processor::utils::save_account, state::*};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::error::StakeError;

pub fn initialize_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    reward_rate: u32,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_iter)?;

    let authority_account = next_account_info(account_iter)?;

    let staking_mint_account = next_account_info(account_iter)?;

    let reward_mint_account = next_account_info(account_iter)?;

    let vault_account = next_account_info(account_iter)?;

    let reward_vault_account = next_account_info(account_iter)?;

    if pool_account.owner != program_id {
        msg!("pool account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    };
    if !authority_account.is_signer {
        msg!("pool account is not owned by this program");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // initialize pool state
    let now = Clock::get()?.unix_timestamp;
    let pool = Pool {
        authority: *authority_account.key,
        staking_mint: *staking_mint_account.key,
        reward_mint: *reward_mint_account.key,
        vault: *vault_account.key,
        vault_staked_tokens: *staking_mint_account.key,
        vault_reward_token: *reward_vault_account.key,
        reward_rate: reward_rate,
        total_staked: 0,
        last_update_time: now,
    };

    save_account(pool_account, &pool)?;
    msg!("pool in initialized with reward rate : {}", reward_rate);
    Ok(())
}
