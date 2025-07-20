use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_instruction,
    program::invoke,
};

pub fn pay_via_cpi(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let payer = next_account_info(account_iter)?;
    let receiver = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let txn = system_instruction::transfer(payer.key, receiver.key, amount);
    invoke(
        &txn,
        &[payer.clone(), receiver.clone(), system_program.clone()],
    );
    Ok(())
}

pub fn pay_via_account(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let payer = next_account_info(account_iter)?;
    let reciever = next_account_info(account_iter)?;

    // transfer lamport == amount from one token to other
    **payer.try_borrow_mut_lamports()? -= amount;
    **reciever.try_borrow_mut_lamports()? += amount;
    Ok(())
}
