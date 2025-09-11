use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod message_account {
 

    use super::*;

    pub fn initialize(ctx : Context<Initialize> , message : String) -> Result<()> {
        let account = &mut ctx.accounts.MessageAccount;
        account.message = message;
        Ok(())
    }
 
    pub fn update(ctx : Context<Update> , input : String) -> Result<()> {
      let account = &mut ctx.accounts.messageaccount;
      account.message = input;
      Ok(())
    }

    pub fn delete(_ctx : Context<Delete>) -> Result<()> {
        Ok(())
    }

    #[account]
    pub struct Message {
        pub message: String,
    }

    impl Message {
        fn required_space(len : usize) -> usize {
         8 + 8 + len 
        }
    }

    #[derive(Accounts)]
    #[instruction(message : String)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub payer : Signer<'info>,
        #[account(init, 
        payer = payer,
        space = Message::required_space(message.len()))]
        pub messageaccount : Account<'info, Message>,
        pub system_program : Program<'info , System>
    } 
    
    #[derive(Accounts)]
    #[instruction(message: String)]
    pub struct Update<'info> {
        #[account(mut)]
        pub signer : Signer<'info>,

        #[account(
            mut,
            realloc = Message::required_space(message.len()),
            realloc::payer = signer,
            realloc::zero = true,
        )]

        pub messageaccount : Account<'info, Message>,
        pub system_program : Program<'info , System>
    }
    #[derive(Accounts)]
    pub struct Delete<'info> {
        #[account(mut)]
        pub payer : Signer<'info>,
        #[account(
            payer,
            close = user 
        )]
        pub messageaccount : Account<'info, Message>
    }
}
