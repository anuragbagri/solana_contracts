use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{self, AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
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
    let instructions = Instruction::try_from_slice(instruction)?;

    let mut account_iter = accounts.iter();
    let recipient = next_account_info(&mut account_iter)?; // address that recieves payment

    let account_to_delete = next_account_info(&mut account_iter)?; // data account to delete

    let signer = next_account_info(&mut account_iter)?;
    // authority

    match instructions {
        Instruction::CloseAccount => {
            // check if the account is owned by this program
            if *account_to_delete.owner != program_id {
                msg!("Account is not onwed by this program");
                return Err(ProgramError::IncorrectProgramId);
            }

            if !signer.is_signer {
                msg!("missing required signer");
                return Err(ProgramError::MissingRequiredSignature);
            }

            // signer must be the recipient
            if signer.key != recipient.key {
                msg!("unauthorized signer");
                return Err(ProgramError::IllegalOwner);
            }

            // transfer lamports
            **recipient.lamports.borrow_mut() += **account_to_delete.lamports.borrow();
            **account_to_delete.lamports.borrow_mut() = 0;

            //clear data in account_to_delete
            let mut data = account_to_delete.try_borrow_mut_data()?;
            for byte in data.iter_mut() {
                *byte = 0;
            }

            msg!("accounts closed and lamports transferred");
            Ok(())
        }
    }
}
