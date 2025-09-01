use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::processor::initialize_pool;
use crate::{error::StakeError, instruction::StakingInstruction};

// processor struct
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &AccountInfo,
        instruction_data: &[u8],
    ) -> ProgramError {
        let instruction = StakingInstruction::try_from_slice(instruction_data)
            .map_err(|_| StakeError::InValidInstruction)?;

        match instruction {
            StakingInstruction::InitializePool { amount } => {
                Self::initialize_pool(program_id, accounts, reward_rate)
            }
            StakingInstruction::Stake { amount } => {
                Self::process_stake(program_id, accounts, amount)
            }
            StakingInstruction::Unstake { amount } => {
                Self::process_unstake(program_id, accounts, amount)
            }
            StakingInstruction::ClaimReward => Self::process_claim(program_id, accounts),
        }
    }
}
