use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint:: ProgramResult, example_mocks::solana_sdk::system_instruction, msg, program::invoke_signed, pubkey::Pubkey, rent::Rent, sysvar::Sysvar
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize)];

pub struct BaseAccount {
    pub message_count : u32
}

#[derive(BorshDeserialize, BorshSerialize)];
pub struct MessageAccount {
    pub content : String
}


entrypoint!(process_message);

fn process_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let signer = next_account_info(account_iter)?;
    let base_account = next_account_info  (account_iter)?;     // base pda 
    let message_account = next_account_info(account_iter)?;     // pda for msg
    let system_account = next_account_info(account_iter)?;

    // deserialize base_account
    let mut base_data = if base_account.data_len() > 0 {
        BaseAccount::try_from_slice(&base_account.data.borrow())?
    } else {
        BaseAccount {
            message_count : 0 ,
        }
    };

    // derive pda
    let (pda , bump ) = Pubkey::find_program_address(
        &[b"pda", &base_account.key.to_bytes() ,&base_data.message_count.to_le_bytes()], program_id);

    if pda!= *message_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // create the new message account 
    let space = 4 + message.len(); 
    let rent = Rent::get()?.minimum_balance(space);
    let txn = system_instruction::create_account(signer.key, &pda, rent, space, base_account.key);


    invoke_signed(&txn, &[signer.clone(), message_account.clone(), system_account.clone()],
  &[&[b"pda", &base_account.key.to_bytes(), &base_data.message_count.to_be_bytes(), &[bump]]])?;


  //store message 
   let msg_data = MessageAccount { content: message };
    msg_data.serialize(&mut &mut message_account.data.borrow_mut()[..])?;

        // Increment count and store in base
    base_data.message_count += 1;
    base_data.serialize(&mut &mut base_account.data.borrow_mut()[..])?;

    msg!("Stored dynamic message #{}", base_data.message_count);
    Ok(())
}
