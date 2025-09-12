use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_stake(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user_account = next_account_info(account_iter)?;
    let user_stake_account = next_account_info(account_iter)?;
    let pool_account = next_account_info(account_iter)?;

    let user_src_token_account = next_account_info(account_iter)?;

    let vault_token_account = next_account_info(account_iter)?;

    let token_program_account = next_account_info(account_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if pool_account.owner != program_id || user_stake_account.owner != program_id {
        msg!("Pool/UserStake must be owned by program");
        return Err(ProgramError::MissingRequiredSignature);
    };

    if amount == 0 {
        msg!("stake amount should be grated than 0");
        return Err(ProgramError::InvalidInstructionData);
    };
    Ok(())

    // state update and transfer left
}
