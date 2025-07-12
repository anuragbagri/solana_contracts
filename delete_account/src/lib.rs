use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{self, AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction {
    CloseAccount,
}

entrypoint!(process_account);

fn process_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: &[u8],
) -> ProgramResult {
    if system_program::check_id(program_id) {
        return ProgramError::IncorrectProgramId;
    }
    // security check for account
    if accounts.len() < 3 {
        msg!("needs sufficient accounts to process");
        return ProgramError::NotEnoughAccountKeys;
    };
    let account_iter = accounts.iter();
    let payer = next_account_info(account_iter);
    let account_to_delete = next_account_info(account_iter);
    let system_account = next_account_info(account_iter);
    let instructions = Instruction::try_from_slice(instruction)?;

    let lamports = account_to_delete.lamports();
    // send to payer

    match instructions {
        Instruction::CloseAccount => {
            // account delete step
        }
    };
    ok(())
}
