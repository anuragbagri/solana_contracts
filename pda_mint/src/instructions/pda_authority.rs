use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult},
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::state::MintAuthorityPda;

pub fn init_pda(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let mint_authority = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    let (mint_authority_pda, bump) =
        Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX.as_bytes()], program_id);

    if (mint_authority_pda == mint_authority.key) {
        // creating the mint authority
        let mint_auth_txn = system_instruction::create_account(
            payer.key,
            &mint_authority.key,
            Rent::get()?.minimum_balance(MintAuthorityPda::SIZE),
            MintAuthorityPda::SIZE as u64,
            program_id,
        );

        invoke_signed(
            &mint_auth_txn,
            &[
                mint_authority.clone(),
                payer.clone(),
                system_program.clone(),
            ],
            &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
        )?;
    } else {
        msg!("the pda address does not the mint_atuhority address");
    }
    let data = MintAuthorityPda { bump }; // bump to be stored inside of the pda
    data.serialize(&mut &mut mint_authority.data.borrow_mut()[..])?;
    Ok(())
}
