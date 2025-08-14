use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::AmmErr, instruction::AmmInstruction, state::Pool};

pub struct processor;

impl processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instructions = AmmInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instructions {
            AmmInstruction::InitializePool { fee_bps } => {
                Self::initialize_pool(program_id, accounts, fee_bps)
            }
            AmmInstruction::AddLiquidity {
                amount_a,
                amount_b,
                min_lp,
            } => Self::add_liquidity(program_id, accounts, amount_a, amount_a, min_lp),
            AmmInstruction::RemoveLiquidity {
                lp_amount,
                min_a,
                min_b,
            } => Self::remove_liquidity(program_id, accounts, min_a, min_b),
            AmmInstruction::SwapExactIn { amount_in, min_out } => {
                Self::swap_exact_in(program_id, accounts, amount_in, min_out)
            }
        }
    }

    fn authority_seeds<'a>(pool_key: &Pubkey, bump: U8) -> [&'a [u8]; 3] {
        [b"authority", pool_key.as_ref(), &[bump][..]]
    }
}
