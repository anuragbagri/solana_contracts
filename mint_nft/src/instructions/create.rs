use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke,
    rent::Rent,
    system_instruction,
};

use spl_token::{instruction as token_instruction, stat::Mint};

use {
    borsh::{BorshDeserialize, BorshSerialize},
    mpl_token_metadata::instruction as mpl_instruction,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenParameters {
    pub nft_title: String,
    pub nft_symbol: String,
    pub nft_url: String,
}

fn create_token(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let parameters = TokenParameters::try_from_slice(instruction_data)?;
    let account_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    // creating the mint account
    let create_account_txn = system_instruction::create_account(
        payer.key,
        mint_account.key,
        (Rent::get()?).minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        token_program.key,
    );

    invoke(
        &create_account_txn,
        &[
            payer.clone(),
            mint_account.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // initialize the mint account as mint
    msg!("initialize mint account....");
    msg!("mint : {}", mint_account.key);

    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            Some(mint_authority.key),
            0, // decimal
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            toke_program.clone(),
        ],
    )?;

    msg!(creating metadata account);
    let create_metadata_txn = mpl_instruction::create_metadata_account_v3(
        *token_metadata_program.key,
        *metadata_account.key,
        *mint_account.key,
        *mint_authority.key,
        *payer.key,
        *mint_authority.key,
        parameters.nft_title,
        parameters.nft_symbol,
        parameters.nft_url,
        None,
        0,
        true,
        false,
        None,
        None,
        None,
    );

    invoke(
        &create_metadata_txn,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
        ],
    )?;

    OK(())
}
