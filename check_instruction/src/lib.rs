use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_instruction,
    msg,
    program::invoke,
    program_error::ProgramError,
    rent::Rent,
    system_program,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &mut [AccountInfo],
    instruction: &[u8],
    space: usize,
) -> ProgramResult {
    // first lets check that whether the program id from the instruction is program id of the our program or not
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    };

    if accounts.len < 4 {
        msg!("this accounts instruction array requires 4 accounts");
        msg!(" payer , account_To_create , account_to_change , system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Accounts passed in the vector must of the following order
    let account_iter = accounts.iter();
    let payer = next_account_info(account_iter)?;
    let account_to_create = next_account_info(account_iter)?;
    let account_to_change = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // check accounttocreate is initialized or not
    msg!("account to create is {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("account to create has already been initialized ");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // (create account) if data account does not exists
    let rent = Rent::get()?;
    let lamports_req = rent.minimum_balance(space);

    let create_tnx = system_instruction::create_account(
        payer.key,
        account_to_create.key,
        lamports_req,
        space as u64,
        program_id,
    );

    // invoke system_program to create a data account on current program behalf(your program account/contract)
    invoke(
        &create_tnx,
        &[
            payer.clone(),
            account_to_create.clone(),
            system_program.clone(),
        ],
    )?;

    // make sure accounttochange has already been initialized
    msg!("account to change is {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("account not initialized");
        return Err(ProgramError::UninitializedAccount);
    };

    // check account_to_change is owned by system program or not
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct porgram id");
        return Err(ProgrmaError::IncorrectProgramId);
    }
}
