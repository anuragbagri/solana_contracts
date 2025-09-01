use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::{error::StakeError, instruction::StakingInstruction};
use crate::processor::initialize_pool;

// processor struct
pub struct Processor; 

impl Processor {
    pub fn process (
        program_id : &Pubkey,
        accounts : &AccountInfo,
        instruction_data : &[u8]
    ) -> ProgramError {
        let instruction = StakingInstruction::try_from_slice(instruction_data).map_err(|_| StakeError::InValidInstruction)?;

        match instruction {
            StakingInstruction::InitializePool { amount } => {
                Self::
            },
            StakingInstruction::Stake { amount }
            => {
                Self::
            },
            StakingInstruction::Unstake { amount } => {
                Self::
            }
            StakingInstruction::ClaimReward => {
                Self::
            }
        }
    }
}