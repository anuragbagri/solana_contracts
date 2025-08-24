use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ReserveParams {
    pub base_rate_per_year_bps: u16,
    pub slope1_bps: u16,
    pub slope2_bps: u16,
    pub kink_bps: u16,
    pub ltv_bps: u16,
    pub liquidation_threshold_bps: u16,
    pub liquidation_bonus_bps: u16,
    pub decimals: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum LendInstruction {
    InitMarket { oracle_program_id: Pubkey },
    Deposit { amount: u64 },

    Withdraw { amount: u64, max_slippage_bps: u16 },

    Borrow { amount: u64 },

    Repay { amount: u64 },
    Liquidate { repay_amount: u64 },

    Pause {},
    Unpause {},
}
