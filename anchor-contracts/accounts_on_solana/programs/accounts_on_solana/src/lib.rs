use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, InitializeAccount, Mint, MintTo, Token, TokenAccount};

declare_id!("weneedtoputtheprogramidofthebelowprogram");

#[program]
pub mod account {
    use super::*;

    pub fn create_data_account(ctx: Context<CreateDataAccount>, value: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.value = value;
        Ok(())
    }

    pub fn create_pda_account(ctx: Context<CreatePdaAccount>, value: u64) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.value = value;
        Ok(())
    }

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        Ok(())
    }

    pub fn create_ata(ctx: Context<CreateAta>) -> Result<()> {
        Ok(())
    }
    #[account]
    pub struct DataAccount {
        pub value: u64,
    }

    #[account]
    pub struct PdaAccount {
        pub value: u64,
    }

    #[derive(Accounts)]
    pub struct CreateDataAccount<'info> {
        #[account(init ,payer=user , space = 8+8)]
        pub data_account: Account<'info, DataAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct CreatePdaAccount<'info> {
        #[account(init, payer = user, space = 8 + 8 , seeds = [b"pda" + user.key().as_ref()],bump)]
        pub pda_account: Account<'info, PdaAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct CreateMint<'info> {
        #[account(init, payer=user , mint::decimals=6,mint::authority=user)]
        pub mint: Account<'info, Mint>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    #[derive(Accounts)]
    pub struct CreateAta<'info> {
        #[account(init, payer = user ,associated_token::mint = mint ,
        associated_token::authority = user)]
        pub ata: Account<'info, TokenAccount>,
        pub mint: Account<'info, Mint>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub associated_token_program: Program<'info, AssociatedToken>,
        pub rent: Sysvar<'info, Rent>,
    }
}
