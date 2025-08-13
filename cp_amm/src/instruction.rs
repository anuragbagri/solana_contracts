pub enum AmmInstruction {
    InitializePool {
        fee_bps: u16,
    },
    AddLiquidity {
        amount_a: u64,
        amount_b: u64,
        min_lp: u64,
    },
    RemoveLiquidity {
        lp_amount: u64,
        min_a: u64,
        min_b: u64,
    },
    SwapExactIn {
        amount_in: u64,
        min_out: u64,
    },
}
