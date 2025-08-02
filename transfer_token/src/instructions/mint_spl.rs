use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg, program::invoke,
};
use spl_associated_token_account::instruction ;
use spl_token::instruction as token_instruction ;

pub struct MintQunatity {
    pub quantity : u64
};
fn mint_token(accounts: &[AccountInfo] , token_quantity : MintQunatity) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let associated_token_account = next_account_info(account_iter)?;
    let signer = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let associated_token_account = next_account_info(account_iter)?;
     
     if associated_token_account.lamports() == 0 {
        msg!("creating the associated token account ");
        let ata_txn = instruction::create_associated_token_account(signer.key, signer.key, mint_account.key, token_program.key);

        invoke(&ata_txn, &[associated_token_account.clone(), mint_account.clone(), mint_authority.clone(), signer.clone(), system_program.clone(),token_program.clone(),associated_token_account.clone()])?;
     } else {
        msg!("associated account already exist : {}", associated_token_account.key);
     };
     msg!("minting {} token to the associated token account ", token_quantity.quantity);
     
     let mint_txn = token_instruction::mint_to(
        token_program.key, mint_account.key, associated_token_account.key,
        signer.key, 
        &[signer.key],
        token_quantity.quantity);

    invoke(&mint_txn, &[mint_account.clone(), mint_authority.clone(), associated_token_account.clone(), token_program.clone()])?;
     
    msg!(" tokens minted to ata account of the signer successfully");
    Ok(())
}
