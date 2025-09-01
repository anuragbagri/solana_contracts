// input instruction by user
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum StakingInstruction {
    InitializePool { amount: u64 },
    Stake { amount: u64 },
    Unstake { amount: u64 },
    ClaimReward,
}
