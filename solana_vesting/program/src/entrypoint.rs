use solana_program::{
    account_info::{AccountInfo, next_account_info},
    config::program,
    entrypoint::{self, ProgramResult},
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
