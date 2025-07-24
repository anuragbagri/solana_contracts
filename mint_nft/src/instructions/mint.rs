use {
    mpl_token_metadata::instruction as mpl_instruction,
    solana_program::{
        account_info::{AccountInfo, next_account_info},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
    },
    spl_associated_token_account::instruction as associated_token_account_instruction,
    spl_token::instruction as token_instruction,
};

pub fn mint_to(accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    // accounts
    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let edition_account = next_account_info(account_iter)?;
    let associated_token_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let associated_token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    if associated_token_account.lamport() == 0 {
        msg!(
            "creating associated token account at {}",
            associated_token_account.key
        );

        let create_ata_txn = associated_token_account_instruction::create_associated_token_account(
            associated_token_account.key,
            payer.key,
            mint_account.key,
            token_program.key,
        );

        invoke(
            &create_ata_txn,
            &[
                mint_account.clone(),
                associated_token_account.clone(),
                payer.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("ata exists");
    }

    // minting nft to user wallet (ata account)
    let mint_nft_ata = token_instruction::mint_to(
        token_program.key,
        mint_program.key,
        associated_token_account.key,
        mint_authority.key,
        &[mint_auhtority.key],
        1, // mint quantity
    );

    invoke(
        &mint_nft_ata,
        &[
            mint_account.close(),
            mint_authority.clone(),
            associated_token_account.clone(),
            token_program.clone(),
        ],
    )?;
    // creating the edition account
    let edition_account_txn = mpl_instruction::create_master_edition_v3(
        *token_metadata_program.key,
        *edition_account.key,
        *mint_account.key,
        *mint_authority.key,
        *metadata_account.key,
        *payer.key,
        Some(1),
    );

    invoke(
        &edition_account_txn,
        &[
            edition_account.clone(),
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
            rent.clone(),
        ],
    )?;

    Ok(())
}
