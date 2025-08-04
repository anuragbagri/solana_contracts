use mpl_token_metadata::instructions::{self, *};
use mpl_token_metadata::state::Mint;

use solana_program::program::invoke_signed;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use borsh::{BorshDeserialize, BorshSerialize};

use spl_token::instruction as token_instructions;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenArgs {
    pub nft_name: String,
    pub nft_symbol: String,
    pub nft_url: String,
}

// creating the mint for the nft
fn create_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token_args: TokenArgs,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let signer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    // checking the mint_account
    if mint_account.lamports() == 0 {
        msg!("creating the mint account");
        let mint_txn = system_instruction::create_account(
            signer.key,
            mint_account.key,
            Rent::get()?.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        );
        invoke(
            &mint_txn,
            &[
                mint_account.clone(),
                signer.clone(),
                system_program.clone(),
                token_program.clone(),
            ],
        )?;
    } else {
        msg!("mint_account is : {}", mint_account.key);
    }

    // initialize mint account
    let init_mint_account = token_instructions::initialize_mint(
        token_program.key,
        mint_account.key,
        mint_authority.key,
        Some(mint_authority.key),
        0,
    )?; // nft 

    invoke(
        &init_mint_account,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            signer.clone(),
        ],
    )?;

    // cretaing the metadata account
    let metadata_txn = instructions::create_metadata_account_v3(
        *token_metadata_program.key,
        *metadata_account.key,
        *mint_account.key,
        *mint_authority.key,
        *signer.key,
        Some(*mint_authority.key),
        token_args.nft_name,
        token_args.nft_symbol,
        token_args.nft_url,
        None,  // no creators
        0,     // no seller fee basis points,
        true,  // not mutable
        false, // not update authority
        None,  // no collection
        None,  // no uses
        None,  // no collection details)
    );

    invoke_signed(
        &metadata_txn,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            signer.clone(),
            token_metadata_program.clone(),
            signer.clone(),
        ],
        &[&[/* to be filled */]],
    )?;

    msg!("tokenmint created successfully");
    Ok(())
}
