use anchor_lang::prelude::*;

#[derive(InitSpace)]
pub struct PayoutScheduleStruct {
    interval: i64,
    next_payout_ts: i64,
    active: bool,
}
