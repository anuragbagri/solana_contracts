use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    //    system_instruction, // Deprecated, replaced by solana_system_interface
    solana_system_interface,
    sysvar::Sysvar,
};

entrypoint!(process_rent);

fn process_rent(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let signer = next_account_info(account_iter)?;
    let account_to_create = next_account_info(account_iter)?;
    let system_account = next_account_info(account_iter)?;

    msg!("account to create: {}", &account_to_create.key.to_string());

    let rent = Rent::get()?;
    let rent_requuired = rent.minimum_balance(instruction_data.len());

    use solana_system_interface::system_instruction;

    let txn = system_instruction::create_account(
        signer.key,
        account_to_create.key,
        rent_requuired,
        instruction_data.len() as u64,
        &program_id,
    );

    invoke(
        &txn,
        &[
            signer.clone(),
            account_to_create.clone(),
            system_account.clone(),
        ],
    );
    Ok(())
}
