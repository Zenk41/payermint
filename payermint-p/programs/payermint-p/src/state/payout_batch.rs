use anchor_lang::prelude::*;

#[account]
pub struct PayrollBatch {
    pub vault: Pubkey,
    pub batch_id: u64,
    pub timestamp: i64,
    pub total_amount: u64,
    pub service_fee: u64,
    pub payout_count: u32,
    pub finalized: bool,
    pub bump: u8,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PayoutSchedule {
    pub interval: i64,
    pub next_payout_ts: i64,
    pub active: bool,
}