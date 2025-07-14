// Program A: caller_program/src/lib.rs

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Program A: Invoking Program B");

    let accounts_iter = &mut accounts.iter();
    let caller = next_account_info(accounts_iter)?; // payer/signer
    let counter_account = next_account_info(accounts_iter)?;
    let program_b = next_account_info(accounts_iter)?; // Program B address

    let ix = Instruction {
        program_id: *program_b.key,
        accounts: vec![
            AccountMeta::new(*counter_account.key, false), // counter account
        ],
        data: vec![], // Program B doesn't expect any instruction data
    };

    // Perform CPI
    invoke(&ix, &[counter_account.clone(), program_b.clone()])?;

    Ok(())
}
