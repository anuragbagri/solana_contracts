use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::instruction::Mint;
use mpl_token_metadata::instructions::create_metadata_accounts_v3;

use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_instruction,
    msg,
    program::invoke,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_token::instruction::TokenInstruction;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenParamter {
    pub token_title: String,
    pub token_sym: String,
    pub token_url: String,
    pub decimals: u8,
}

pub fn create_mint(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let token_arguments = TokenParamter::try_from_slice(instruction_data)?;

    let account_iter = &mut accounts.iter();

    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let token_metadata_account = next_account_info(account_iter)?;

    msg!("creating the mint account at :{}", mint_account.key);
    let mint_account_txn = system_instruction::create_account(
        payer.key,
        mint_account.key,
        (Rent::get()?).minimum_balance(Mint::LEN),
        Mint::LEN as u64,
        token_program.key,
    );

    invoke(
        &mint_account_txn,
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // initializing the mint account
    let initialize_mint_ix = Instruction {
        program_id: *token_program.key,
        accounts: vec![
            AccountMeta::new(*mint_account.key, false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
        ],
        data: TokenInstruction::InitializeMint {
            decimals: 9,
            mint_authority: *mint_authority.key,
            freeze_authority: Some(*mint_authority.key).into(),
        }
        .pack(), // serialize
    };

    invoke(
        &initialize_mint_ix,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
        ],
    )?;

    // now creating in metadata account for mint account
    let metadata_account_txn = create_metadata_accounts_v3(
        *token_program.key,
        *metadata_account.key,
        *mint_account.key,
        *mint_authority.key,
        *payer.key,
        Some(*mint_authority.key),
        token_arguments.token_title,
        token_arguments.token_sym,
        token_arguments.token_url,
        None,
        0,
        true,
        fals,
        None,
        None,
        None,
    );

    invoke(
        &metadata_account_txn,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            mint_authority.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}
