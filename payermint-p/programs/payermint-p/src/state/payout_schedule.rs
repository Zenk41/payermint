use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct PayoutSchedule {
    interval: i64,
    next_payout_ts: i64,
    active: bool,
}
