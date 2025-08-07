use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

use crate::instructions::{
    create::create_mint,
    pda_authority::init_pda,
    mint::mint_to
}
use borsh::{BorshDeserialize , BorshSerialize};
#[derive(BorshDeserialize,BorshSerialize)]
enum MyInstruction {
    CreatePdaAuthority,
    CreateNft(TokenArgs),
    MintNft
}
pub fn process_instruction(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
) -> ProgramResult {
    let instructions = MyInstruction::try_from_slice(instruction_data)?;

    match instructions {
      MyInstruction::CreatePdaAuthority => init_pda(program_id, accounts),
      MyInstruction::CreateNft(args) => create_mint(program_id, accounts, args),
      MyInstruction::MintNft => mint_to(program_id, accounts)
    }

}