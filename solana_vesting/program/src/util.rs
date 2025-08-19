// vesting pda account - to hold all the information vesting
// escrow ata - to hold the token
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use spl_associated_token_account::{
    instruction::create_associated_token_account,
    solana_program::example_mocks::solana_sdk::system_instruction,
};

use crate::error::VestingError;

pub fn create_pda_account(
    payer: &AccountInfo,
    new_pda: &AccountInfo,
    program_id: &Pubkey,
    seeds: &[&[u8]],
    space: usize,
) -> ProgramResult {
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);
    let pda_instruction = system_instruction::create_account(
        payer.key,
        new_pda.key,
        lamports,
        space as u64,
        program_id.key,
    );

    invoke_signed(
        &pda_instruction,
        &[new_pda.clone(), payer.clone()],
        &[seeds],
    )?;
    Ok(())
}

pub fn create_escrow_ata(
    payer: &AccountInfo,
    owner_pda: &AccountInfo,
    mint_account: &AccountInfo,
    ata_account: &AccountInfo,
    ata_program: &AccountInfo,
    token_program: &AccountInfo,
    system_program: &AccountInfo,
    rent_sysvar: &AccountInfo,
) -> ProgramResult {
    let ata_instruction = create_associated_token_account(
        payer.key,
        owner_pda.key,
        mint_account.key,
        token_program.key,
    ); // owner is vesting pda

    invoke(
        &ata_instruction,
        &[
            payer.clone(),
            ata_account.clone(),
            owner_pda.clone(),
            mint_account.clone(),
            system_program.clone(),
            ata_program.clone(),
            rent_sysvar.clone(),
        ],
    )
}

// simple rent exempt check
pub fn rent_exempt(account: &AccountInfo) -> ProgramResult {
    let rent = Rent::get()?;
    if !rent.is_exempt(account.lamports(), account.data_len()) {
        return Err(VestingError::NotRentExempt.into());
    }
    Ok(())
}
