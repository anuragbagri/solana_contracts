use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct Pool {
    pub is_initialized: bool,
    pub authority_bump: u8,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub vault_a: Pubkey, // token account owned by authority pda
    pub vault_b: Pubkey, // same
    pub lp_mint: Pubkey, // mint whoes mint auth= authority pda
    pub fee_bps: u16,
    pub total_lp_supply: u64,
}

impl Pool {
    pub const LEN: usize = /* not sized for now */ ;
}
