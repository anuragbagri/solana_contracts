use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{BumpAllocator, ProgramResult},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
};
use spl_token::instruction;

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

    fn initialize_pool(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        fee_bps: u16,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let signer = next_account_info(account_iter)?; // this will pay for all accounts created while creating a pool
        let pool_account = next_account_info(account_iter)?; // main pool account that stores pool metadata
        let token_a_mint = next_account_info(account_iter)?; // mint account for token a(usdc) (should be spl)
        let token_b_mint = next_account_info(account_iter)?; // mint account for token b (sol)
        let vault_a = next_account_info(account_iter)?; // account where actual liquidity exist for usdc
        let vault_b = next_account_info(account_iter)?;
        // owner must be the pda authority

        let lp_mint = next_account_info(account_iter)?; //mint accont for providing the lp token 
        let _rent = next_account_info(account_iter)?;
        let _token_program = next_account_info(account_iter)?;

        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if pool_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        // deriving the pda authority for this pool
        let (authority_pda, bump) =
            Pubkey::find_program_address(&[b"authority", pool_account.key.as_ref()], program_id);

        // validate vault owner and the lp_mint authority (optional )
        let mut pool = Pool::try_from_slice(&pool_account.data.borrow()).unwrap_or(Pool {
            is_initialized: false,
            authority_bump: 0,
            token_a_mint: Pubkey::default(),
            token_b_mint: Pubkey::default(),
            vault_a: Pubkey::default(),
            vault_b: Pubkey::default(),
            lp_mint: Pubkey::default(),
            fee_bps: 0,
            total_lp_supply: 0,
        });

        if pool.is_initialized {
            return Err(AmmErr::AlreadyInitialized.into());
        };

        pool.is_initialized = true;
        pool.authority_bump = bump;
        pool.token_a_mint = *token_a_mint.key;
        pool.token_b_mint = *token_b_mint.key;
        pool.vault_a = *vault_a.key;
        pool.vault_b = *vault_b.key;
        pool.lp_mint = *lp_mint.key;
        pool.fee_bps = fee_bps;
        pool.total_lp_supply = 0;
        pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?;

        msg!("pool initialized authority {} bump {}", authority_pda, bump);
        Ok(())
    }

    fn add_liquidity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount_a: u64,
        amount_b: u64,
        min_lp: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let signer = next_account_info(account_iter)?;
        let user_ata_a = next_account_info(account_iter)?; // usdc provider 
        let user_ata_b = next_account_info(account_iter)?; // sol provider 
        let pool_account = next_account_info(account_iter)?;
        let vault_a = next_account_info(account_iter)?;
        let vault_b = next_account_info(account_iter)?;
        let lp_mint = next_account_info(account_iter)?;
        let user_lp_ata = next_account_info(account_iter)?; // ata to recieve the lp token 
        let token_program = next_account_info(account_iter)?;

        if signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = Pool::try_from_slice(&pool_account.data.borrow_mut())?;
        if !pool.is_initialized {
            return Err(AmmErr::Uninitialized.into());
        }

        // transfer deposits to the vault
        let instruction_a = instruction::transfer(
            token_program.key,
            user_ata_a.key,
            &pool.vault_a,
            signer.key,
            &[],
            amount_a,
        )?;

        invoke_signed(
            &instruction_a,
            &[
                user_ata_a.clone(),
                vault_a.clone(),
                signer.clone(),
                token_program.clone(),
            ],
            &[],
        )?;

        let instruction_b = instruction::transfer(
            token_program.key,
            user_ata_b.key,
            &pool.vault_b,
            signer.key,
            &[],
            amount_b,
        )?;

        invoke_signed(
            &instruction_b,
            &[
                signer.clone(),
                vault_b.clone(),
                user_ata_b.clone(),
                token_program.clone(),
            ],
            &[],
        )?;
    }
}
