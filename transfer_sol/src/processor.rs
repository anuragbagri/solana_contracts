use crate::instructions::pay_via_account;
use crate::instructions::pay_via_cpi;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Input {
    CpiAccountTransfer(u64),
    ProgramAccountTransfer(u64),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let input = Input::try_from_slice(instruction_data)?;
    match input {
        Input::CpiAccountTransfer(amount) => pay_via_cpi(accounts, amount),
        Input::ProgramAccountTransfer(amount) => pay_via_account(accounts, amount),
    };

    Ok(())
}
