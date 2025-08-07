pub mod instructions;
pub mod processor;
pub mod state;



use solana_program::{
    account_info::{next_account_info,AccountInfo},
    entrypoint::{self, ProgramResult},
    msg,
    pubkey::Pubkey
};
entrypoint!(process_instruction)
fn process_instruction(program_id : &Pubkey , accounts : &[AccountInfo] , instruction_data  : &[u8]) -> ProgramResult {
   processor::process_instruction(program_id, accounts, instruction_data);
}