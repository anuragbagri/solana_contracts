use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar
};
use spl_token::instruction;


use crate::{error::LendError, math::{ray_div, ray_mul}, state::{Market, Obligations, Reserve}};

// this fn updates the stae of reserve pool over time 
fn accure_reserve(reserve: &mut Reserve, now_ts: i64) -> ProgramResult {
    if now_ts <= reserve.last_update {
        return ;
    };
    let dt = (now_ts - reserve.last_update) as u64;

    let borrows = ray_mul(reserve.total_scaled_borrows,reserve.borrow_index);
    let deposits  = ray_mul(reserve.total_scaled_deposits, reserve.liquidity_index);
    let liquidity = deposits.saturating_sub(borrows);

    if liquidity == 0 && borrows == 0 {
        reserve.last_update = now_ts;
        return ;
    }

    let utilization_rate = if borrows = 0 { 0 } else { ray_div(borrows, liquidity.saturating_add(borrows))}; 

    // u = borrows / borrows + liquidity 


    // borrow interest rate 
    let borrow_interest_rate = borrow_rate_bps // function to calculate rate 


    Ok(())
}

pub  fn process(program_id : &Pubkey, accounts : &[AccountInfo], amount : u64) ->  ProgramResult {
  let account_iter = &mut accounts.iter(); 
  let market_account = next_account_info(account_iter)?;
  let reserve_account = next_account_info(account_iter)?;
  let obligation_account = next_account_info(account_iter)?;
  let vault_account = next_account_info(account_iter)?;
  let user_account = next_account_info(account_iter)?;
  let user_ata_account = next_account_info(account_iter)?;
  let token_program = next_account_info(account_iter)?;
  let orcale_id = next_account_info(account_iter)?;


  if !user_account.is_signer {
    return  Err(LendError::Unauthorized.into());
  }

  if market_account.owner != program_id || reserve_account.owner != program_id || obligation_account.owner != program_id {
    return Err(LendError::MismatchedAccounts.into());
  }

  let market = Market::try_from_slice(&market_account.try_borrow_data()?).map_err(|_| ProgramError::InvalidAccountData)?;

  if market.paused {
    return Err(LendError::Paused.into());
  }
   
  // market and reserve state and obligations
  let mut reserve = Reserve::try_from_slice(&reserve_account.try_borrow_data()?).map_err( |_| ProgramError::InvalidAccountData)?;

  if reserve.market != *market_account.key {
    return Err(LendError::MismatchedAccounts.into());
  }

  let obligation = Obligations::try_from_slice(&obligation_account.try_borrow_data()?).map_err(|_| ProgramError::InvalidAccountData)?;

  // accure 
  let time = solana_program::clock::Clock::get()?.unix_timestamp;
  accure_reserve(&mut reserve, time);

  // transfer tokens user -> vault 
  let transfer_instruction = instruction::transfer(token_program.key, user_account.key, &reserve.vault, user_account.key, &[], amount)?;

  invoke(&transfer_instruction, &[user_ata_account.clone(), vault_account.clone(),user_account.clone(),token_program.clone() ])?;

  



}
 