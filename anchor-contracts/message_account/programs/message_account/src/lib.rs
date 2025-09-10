use anchor_lang::prelude::*;

declare_id!();

#[program]
pub mod MessageAccount {
    use super::*;

    pub fn Initialize() -> Result<()> {}

    pub fn Update() -> Result<()> {}

    pub fn Delete() -> Result<()> {}

    #[account]
    pub struct MessageAccount {
        pub message: String,
    }

    #[derive(Accounts)]
    #[instruction(message : String)]
    pub struct Initialize<'info> {}

    #[derive(Accounts)]
    #[instruction(message : String)]
    pub struct Update<'info> {}

    #[derive(Accounts)]
    #[instruction(message : String)]
    pub struct Delete<'info> {}
}
