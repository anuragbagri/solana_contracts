use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8], // Not used in this example
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let signer = next_account_info(account_iter)?; // payer account (signer)
    let account_to_create = next_account_info(account_iter)?; // new account to be created
    let system_program = next_account_info(account_iter)?; // system_program (must match system_program::ID)

    let space: usize = 100; // Replace with required size
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    msg!("Creating a new account...");
    let txn = system_instruction::create_account(
        signer.key,
        account_to_create.key,
        lamports,
        space as u64,
        program_id,
    );

    invoke(
        &txn,
        &[
            signer.clone(),
            account_to_create.clone(),
            system_program.clone(),
        ],
    )?;

    msg!("Account created successfully.");
    Ok(())
}
