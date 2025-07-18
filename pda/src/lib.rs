use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, solana_program::system_instruction, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar
};

use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Data {
    roll_no : u32,
    score : u32
},
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
  let  account_iter =&mut  accounts.iter();
  let signer = next_account_info( account_iter)?;
  let pda_account = next_account_info( account_iter)?;
  let system_program = next_account_info(account_iter)?;
  
  let data = Data::try_from_slice(instruction_data)?;

  // derive pda
  let (pda , bump ) = Pubkey::find_program_address(&[b"pda" , signer.key.as_ref()], program_id);

  if *pda_account.key != pda {
    msg!("incorrect arguments");
    return Err(ProgramError::InvalidArgument);
  };

  let rent = Rent::get()?;
  let data_len = instruction_data.len();
  let lamports_req = rent.minimum_balance(data_len);
   
  let instruction = system_instruction::create_account(
    signer.key, 
    &pda,
     lamports_req, 
     data_len, 
     program_id);

  let signer_seed = &[b"pda" , signer.key.as_ref() ,&[bump]];
  invoke_signed(
    &instruction, 
    &[signer.clone(), pda_account.clone(), system_program.clone()], 
    &[signer_seed]
)?;

let pda_data = pda_account.try_borrow_mut_data()?;
data.serialize(&mut *pda_data);

Ok()
}
