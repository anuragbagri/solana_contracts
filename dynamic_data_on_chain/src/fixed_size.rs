use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, example_mocks::solana_sdk::system_instruction, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar
};

use borsh::{BorshDeserialize , BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)];

entrypoint!(process_instruction); 

pub struct Size {
    size : u64
}

fn process_instruction(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
) -> ProgramResult {
     // in this program , create a account(data account) that is very big 
     let account_iter = &mut accounts.iter();
     let signer = next_account_info(account_iter)?;
     let account_to_create = next_account_info(account_iter)?;
     let system_account = next_account_info(account_iter)?;
    
     let instruction = Size::try_from_slice(instruction_data)?;
     
     let rent = Rent::get()?;
     let lamports_req = rent.minimum_balance(instruction::size);

     let txn = system_instruction::create_account(signer.key,
        account_to_create, lamports_req,instruction::size , program_id);

    invoke(&txn, &[signer.clone() , account_to_create.clone(), system_account.clone()]);
    Ok()
     
}