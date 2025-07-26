use solana_program::{
    account_info::{next_account_info, AccountInfo}, msg, program::invoke, pubkey::Pubkey

};
use spl_associated_token_account::instruction::AssociatedTokenAccountInstruction;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_token::instruction::TokenInstruction;


pub fn mint_nft(accounts : &[AccountInfo]) ->  {
    let account_iter =&mut accounts.iter();

    let mint_account = next_account_info(account_iter)?;
    let mint_authority = next_account_info(account_iter)?;
    let metadata_account = next_account_info(account_iter)?;
    let edition_account = next_account_info(account_iter)?;
    let associated_token_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_account = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let assocaited_token_program = next_account_info(account_iter)?;
    let token_metadata_program = next_account_info(account_iter)?;

    if associated_token_account.lamports() == 0 {
        // creating ata 
        let ata_txn = create_associated_token_account_idempotent(&payer.key, 
        &payer.key, &associated_token_account.key, &assocaited_token_program.key);

        invoke(&[ata_txn], &[payer.clone(), payer.clone(), mint_account.clone(), associated_token_account.clone(),
        system_account.clone(),
        token_program.clone()],
    )?;
    } else {
        msg!("ata already exists");
    };

    // mint the nft to ata 

    let ix = Instruction {
    program_id: token_program::ID,  // token program
    accounts: vec![
        AccountMeta::new(mint_account.key(), false),
        AccountMeta::new(payer.key(), false),
        AccountMeta::new_readonly(mint_authority.key(), true),
    ],
    data: TokenInstruction::MintTo {
        amount: 1, // 1 token = 1 NFT
    }
    .pack(),
};

   invoke(
    &ix,
    &[
        mint_account.clone(),
        payer.clone(),
        mint_authority.clone(),
    ],
)?;



 // creating  a edition account 
 let (edition_account, _) = Pubkey::find_program_address(
    &[
        b"metadata",
        &token_metadata_program::ID.to_bytes(),
        &mint_account.key().to_bytes(),
        b"edition"
    ],
    &token_metadata_program::ID,
);
invoke(
    &mpl_token_metadata::instruction::create_master_edition_v3(
        token_metadata_program::ID,
        edition_account,
        *mint_account.key,
        *mint_authority.key,
        *payer.key,
        *metadata_account.key,
        Some(max_supply), // Option<u64> for max supply
    ),
    &[
        edition_account.clone(),
        mint_account.clone(),
        mint_authority.clone(),
        payer.clone(),
        metadata_account.clone(),
        token_metadata_program.clone(),
        // system_program and rent if required
    ],
)?;
Ok(())
}