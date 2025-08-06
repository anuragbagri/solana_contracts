// mint the nft to ata
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult},
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};

use mpl_token_metadata::instruction as mpl_instruction;
use spl_associated_token_account::instruction as ata_instruction;
use spl_token::instruction;

use crate::{instructions::mint, state::MintAuthorityPda};

pub fn mint_to(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let associated_account = next_account_info(account_iter)?;
    let edition_account = next_account_info(account_iter)?;
    let signer = next_account_info(account_iter)?;
    let system_progra = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let associated_token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    let (mint_authority_pda, bump) =
        Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX.as_bytes()], program_id);
    assert!(mint_authority_pda.eq(mint_authority.key)); // pda already created in pda_authority file

    if associated_account.lamports() == 0 {
        msg!("creating the associated account ");
        let ata_txn = ata_instruction::create_associated_token_account(
            signer.key,
            signer.key,
            mint_account.key,
            token_program.key,
        );

        invoke(
            &ata_txn,
            &[
                mint_account.key,
                associated_account.clone(),
                signer.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;

        msg!("associated token address is : {}", associated_account.key);
    } else {
        msg!(
            "associated token account exists at : {}",
            associated_account.key
        );
    }

    // minting the nft to ata account
    let mint_to = instruction::mint_to(
        token_program.key,
        mint_account.key,
        associated_account.key,
        signer.key,
        &[mint_authority.key],
        1,
    )?;

    invoke_signed(
        &mint_to,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            associated_account.clone(),
            token_program.clone(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;

    // creating the ediiton account
    invoke_signed(
        &mpl_instruction::create_master_edition_v3(
            *token_metadata_program.key, // Program ID
            *edition_account.key,        // Edition
            *mint_account.key,           // Mint
            *mint_authority.key,         // Update Authority
            *mint_authority.key,         // Mint Authority
            *metadata_account.key,       // Metadata
            *signer.key,                 // Payer
            Some(1),                     // Max Supply
        ),
        &[
            edition_account.clone(),
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            signer.clone(),
            token_metadata_program.clone(),
            signer.clone(), // rent
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;

    // nft minted
    Ok(())
}
