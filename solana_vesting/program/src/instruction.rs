use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum VestingInstruction {
    Initialize {
        total_amount: u64,
        start_time: i64,
        cliff_time: i64,
        end_time: i64,
        revocable: bool,
    },

    Claim {},
    CloseAndRevoke {},
}
