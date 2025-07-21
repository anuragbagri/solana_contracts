use std::env::args;

use spl_token::instruction::{initialize_mint, initialize_mint2};
use {
    borsh::{BorshDeserialize, BorshSerialize},
    mpl_token_metadata::instruction as mpl_instruction,
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_token::{instruction as token_instruction, state::Mint},
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateTokenArgs {
    pub token_title: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub token_decimals: u8,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    let arguments = CreateTokenArgs::try_from_slice(instruction_data)?;

    let lamports_required = Rent::get()?.minimum_balance(Mint::LEN);
    //  creating mint account
    msg!("Creating mint account : {}", mint_account.key);
    let txn_mint = system_instruction::create_account(
        payer.key,
        mint_account.key,
        lamports_required,
        Mint::LEN as u64,
        token_program.key,
    );

    invoke(
        &txn_mint,
        &[
            payer.clone(),
            mint_account.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    );

    // initialize the mint_account as the MINT
    let initialize_txn = token_instruction::initialize_mint2(
        token_program.key,
        mint_account.key,
        mint_authority.key,
        Some(mint_authority.key),
        arguments.token_decimals,
    )?;

    invoke(
        &initialize_txn,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
        ],
    )?;

    //creating the metadata account for mint
    msg!("metadata_account key is {}", metadata_account.key);
    let metadata_txn = mpl_instruction::create_metadata_accounts_v3(
        *token_metadata_program.key,
        *metadata_account.key,
        *mint_account.key,
        *mint_authority.key,
        *payer.key,
        *mint_authority.key,
        arguments.token_title,
        arguments.token_symbol,
        arguments.token.uri,
        None,
        0,
        true,
        false,
        None,
        None,
        None,
    );

    invoke(
        &metadata_txn,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
        ],
    )?;

    Ok(())
}
