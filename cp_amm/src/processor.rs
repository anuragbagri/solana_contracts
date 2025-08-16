use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_associated_token_account::tools::account;
use spl_token::{instruction as token_instruction, solana_program::program_pack::Pack};

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
            } => Self::add_liquidity(program_id, accounts, amount_a, amount_b, min_lp),
            AmmInstruction::RemoveLiquidity {
                lp_amount,
                min_a,
                min_b,
            } => Self::remove_liquidity(program_id, accounts, lp_amount, min_a, min_b),
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

        // transfer deposits to the vault a and b
        let instruction_a = token_instruction::transfer(
            token_program.key,
            user_ata_a.key,
            vault_a.key,
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

        let instruction_b = token_instruction::transfer(
            token_program.key,
            user_ata_b.key,
            vault_b.key,
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
        Ok(())
    }

    fn remove_liquidity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lp_amount: u64,
        min_a: u64,
        min_b: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let user = next_account_info(account_iter)?;
        let user_lp_ata = next_account_info(account_iter)?;
        let lp_mint = next_account_info(account_iter)?;
        let pool_account = next_account_info(account_iter)?;
        let vault_a = next_account_info(account_iter)?
        let vault_b = next_account_info(account_iter)?;
        let user_a_ata = next_account_info(account_iter)?;
        let user_b_ata = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature); 
        };
        let mut pool = Pool::try_from_slice(&pool_account.data.borrow_mut())?;
        if pool.total_lp_supply == 0 { return Err(AmmErr::Uninitialized.into()); }

        // burn the lp from user 
        let burn_transaction = token_instruction::burn(token_program.key,user_lp_ata.key , lp_mint.key, user.key, &[], lp_amount as u64);
        invoke_signed(&burn_transaction, &[user_lp_ata.clone() , lp_mint.clone(), user.clone(), token_program.clone()], &[])?;

        // computer proportional amounts from current reserves 
        let va  = spl_token::state::Account::unpack(&vault_a.data.borrow())?;
        let vb = spl_token::state::Account::unpack(&vault_b.data.borrow())?;
        let reserve_a = va.amount as u128;
        let reserve_b = vb.amount as u128;

        let total_lp = pool.total_lp_supply as u128;
        let out_a = (lp_amount as u128).saturating_mul(reserve_a).checked_div(total_lp).ok_or(ProgramError::InvalidInstructionData)? as u128;

        let out_b = (lp_amount as u128).saturating_mul(reserve_b).checked_div(total_lp).ok_or(ProgramError::InvalidInstructionData)? as u128;

        if out_a < min_a || out_b < min_b { return Err(AmmErr::SlippageExceeded.into()) ; 
        };

        // transfer from vaults signed by authority 
        let (auth , bump) = Pubkey::find_program_address(&[b"auhtority", pool_account.key.as_ref()], program_id);
        let seeds = Self::authority_seeds(pool_account, bump);

        
        let transfer_instruction_a: = token_instruction::transfer(token_program.key, vault_a.key, user_ata_a.key, &auth, &[], out_a)?;
        invoke_signed(&transfer_instruction_a, &[vault_a.clone(), user_ata_a.clone(), token_program.clone()], &[&seeds])?;
        let transfer_instruction_b  = token_instruction::transfer(token_program.key, vault_b.key, user_ata_b.key, &auth, &[], out_b)?;
        invoke_signed(&transfer_instruction_b, &[vault_b.clone(), user_ata_b.clone(), token_program.clone()], &[&seeds])?;

        pool.total_lp_supply = pool.total_lp_supply.saturating_sub(lp_amount);
        pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
