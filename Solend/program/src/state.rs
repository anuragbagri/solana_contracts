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

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Position {
    pub reserve: Pubkey,
    pub scaled_amount: u128,
    pub used_as_collateral: bool,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            reserve: Pubkey::default(),
            scaled_amount: 0,
            used_as_collateral: true,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Obligations {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub collaterals: [Position; 8],
    pub borrows: [Position; 8],
    pub bump: u8,
}

impl Obligations {
    pub fn upsert_position(
        slots: &mut [Position; 8],
        reserve: Pubkey,
        delta_scaled: i128,
        use_as_collateral: bool,
    ) {
        // finding existin slot or empty slot
        let mut idx = None;
        for (i, p) in slots.iter().enumerate() {
            if p.reserve == reserve {
                // already has entry in obligations slot
                idx = Some(i);
                break;
            }
            if p.reserve == Pubkey::default() && idx.is_none() {
                // use it for else
                idx = Some(i);
            }
        }

        // assign  empty slot to upcommig entry
        if let Some(i) = idx {
            let mut p = slots[i];
            if p.reserve == Pubkey::default() {
                p.reserve = reserve;
                p.scaled_amount = 0;
                p.used_as_collateral = use_as_collateral;
            }

            // apply delta
            let cur = p.scaled_amount as i128;
            let next = cur + delta_scaled;
            p.scaled_amount = if next <= 0 { 0 } else { next as u128 };
            //cleanup if zero
            if p.scaled_amount == 0 {
                p = Position::default();
            }
            slots[i] = p;
        }
    }
}
