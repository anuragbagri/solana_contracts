use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
};

use spl_associated_token_account::instruction;
use spl_token::instruction as token_instruction;

pub struct TokenQuantity {
    pub quantity: u64,
}
fn transfer_token(accounts: &[AccountInfo], token_quantity: TokenQuantity) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let mint_account = next_account_info(account_iter)?;
    let from_ata_account = next_account_info(account_iter)?;
    let to_ata_account = next_account_info(account_iter)?;
    let owner = next_account_info(account_iter)?;
    let recipient_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let associated_token_program = next_account_info(account_iter)?;

    if to_ata_account.lamports() == 0 {
        msg!("creating the ata account for the owner");
        let ata_txn = instruction::create_associated_token_account(
            payer.key,
            recipient_account.key,
            mint_account.key,
            token_program.key,
        );

        invoke(
            &ata_txn,
            &[
                mint_account.clone(),
                to_ata_account.clone(),
                recipient_account.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("ata for reciever already exits");
    }

    msg!("ata {} of the receiver is :", to_ata_account.key);

    // transfering token from the woner to recipient
    let transfer_txn = token_instruction::transfer(
        token_program.key,
        from_ata_account.key,
        to_ata_account.key,
        mint_authority.key,
        &[payer.key],
        token_quantity.quantity,
    );

    invoke(
        &transfer_txn,
        &[
            mint_account.clone(),
            from_ata_account.clone(),
            to_ata_account.clone(),
            owner.clone(),
            recipient_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("tokens transferred successfully");

    Ok(())
}
