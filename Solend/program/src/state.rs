use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Market {
    pub admin: Pubkey,
    pub oracle_program_id: Pubkey,
    pub bump: u8,
    pub paused: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Reserve {
    pub market: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub decimals: u8,

    // interest model
    pub base_rate_per_year_bps: u16,
    pub slope1_bps: u16,
    pub slope2_bps: u16,
    pub kink_bps: u16,

    // indices (RAY)
    pub liquidity_index: u128,
    pub borrow_index: u128,
    pub last_update: i64,

    //totals (scaled)
    pub total_scaled_deposits: u128,
    pub total_scaled_borrows: u128,

    // risks
    pub ltv_bps: u16,
    pub liquidation_threshold_bps: u16,
    pub liquidation_bonus_bps: u16,

    pub bump: u8,
    pub paused: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Position {
    pub reserve: Pubkey,
    pub scaled_amount: u128,
    pub used_as_collateral: bool,
}

// set the deafulr values
impl Default for Position {
    fn default() -> Self {
        Self {
            reserve: Pubkey::default(),
            scaled_amount: 0,
            used_as_collateral: true,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Obligation {
    pub market: Pubkey,
    pub onwer: Pubkey,
    pub collaterals: [Position; 8],
    pub borrows: [Position; 8],
    pub bump: u8,
}
