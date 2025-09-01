// this file stores vault state and user state
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Pool {
    pub authority: Pubkey,
    pub staking_mint: Pubkek,
    pub reward_mint: Pubkey,
    pub vault_staked_tokens: Pubkey,
    pub vault_reward_token: Pubkey,
    pub reward_state: Pubkey,
    pub total_staked: Pubkey,
    pub last_update_time: i64,
}

// user staked account
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct UserStake {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub reward_debt: u64,
    pub last_stake_time: i64, // last time staked
}
