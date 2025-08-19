use borsh::{BorshDeserialize, BorshSerialize, de::EnumExt};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_program,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::VestingError, instruction::VestingInstruction, state::*, util::*};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let instructions = VestingInstruction::try_from_slice(data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instructions {
            VestingInstruction::Initialize {
                total_amount,
                start_time,
                cliff_time,
                end_time,
                revocable,
            } => Self::initialize(
                program_id,
                accounts,
                data,
                total_amount,
                start_time,
                cliff_time,
                end_time,
                revocable,
            ),
            VestingInstruction::Claim {} => Self::process_claim(program_id, accounts),
            VestingInstruction::CloseAndRevoke {} => Self::CloseAndRevoke(program_id, accounts),
        }
    }
    fn initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
        total_amount: u64,
        start_time: i64,
        cliff_time: i64,
        end_time: i64,
        revocable: bool,
    ) -> ProgramResult {
        msg!("initialize  + create pda + create escrow ata + fund ");

        let account_iter = &mut accounts.iter();
        let admin = next_account_info(account_iter)?;
        let vesting_pda = next_account_info(account_iter)?;
        let beneficiary = next_account_info(account_iter)?;
        let mint_account = next_account_info(account_iter)?;
        let admin_src_ata = next_account_info(account_iter)?;
        let escrow_ata = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;
        let ata_program = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let rent = next_account_info(account_iter)?;
        if !admin.is_signer {
            return Err(VestingError::Unauthorised.into());
        }

        if system_program.key != system_program::ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        // schedule validation
        if !(start_time <= cliff_time
            && cliff_time <= end_time
            && start_time < end_time
            && total_amount > 0)
        {
            return Err(VestingError::InvalidSchedule.into());
        }

        // derive vesting pda
        let (vesting_pda_expected, vesting_bump) = Pubkey::find_program_address(
            &[
                VESTING_SEED,
                beneficiary.key.as_ref(),
                mint_account.key.as_ref(),
            ],
            program_id,
        );

        if vesting_pda.key != &vesting_pda_expected {
            return Err(VestingError::InvalidSeeds.into());
        };

        // using the helper function
        create_pda_account(
            admin,
            vesting_pda_expected,
            program_id,
            &[
                VESTING_SEED,
                beneficiary.key.as_ref(),
                mint_account.key.as_ref(),
                &[vesting_bump],
            ],
            VestingState::LEN,
        );

        Ok(())
    }
}
