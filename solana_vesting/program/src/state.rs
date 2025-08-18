use borsh::{BorshSerialize , BorshDeserialize};
use solana_program::pubkey::Pubkey;


pub const VESTING_SEED : &[u8] = b"vesting";
pub const ESCROW_SEED : &[u8] = b"escrow";

pub struct VestingState {
    // who receives token 
    pub beneficiary : Pubkey,
    // main admin
    pub admin : Pubkey,
    // token mint (main account where money will be minted)
    pub mint : Pubkey,
    // token holding associated account - owner is vesting pda auth
    pub escrow_ata : Pubkey,

    // schedule 
    pub start_time : i64,
    pub cliff_time : i64,
    pub end_time : i64,
    pub total_amount : u64,
    pub claimed_amount : u64

    // flags 
    pub revocable : bool,

    // bumps 
    pub vesting_bump : u8,
}

impl VestingState {
    pub const LEN : usize =4;// not defined now 
    

    pub fn vested_amount(&self, now: i64) -> u64 {
      if now < self.cliff_time { return  0 ;}
      if now > self.end_time { return self.total_amount ;}

      // linear from start to end time 
      let elapsed = (now - self.start_time ).max(0) as u64;
      let duration = (self.end_time - self.start_time ).max(0) as u64;

      let total = self.total_amount as u64;
      ((elapsed * total) / duration ) as u64 ;  //how many tokens should be available till now
    }

    pub fn claimable(&self , now: i64) -> bool {
         self.vested_amount(now).saturating_sub(self.claimed_amount); // tokens to be vested till current time - already withdraw tokens 
    }

    pub fn fully_claimed(&self) -> bool {
        self.claimed_amount >= self.total_amount
    }
}      