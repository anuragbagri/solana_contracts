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
}
