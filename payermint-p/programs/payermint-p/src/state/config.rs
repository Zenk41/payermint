use anchor_lang::prelude::*;

#[account]
pub struct GlobalConfig {
    pub owner: Pubkey,
    pub treasury: Pubkey,
    pub default_fee_bps: u16,
    pub next_company_id: u64,
    pub bump: u8,
}
