use anchor_lang::prelude::*;

#[account]
pub struct Member {
    pub vault: Pubkey,
    pub wallet: Pubkey,
    pub allocation_bps: Option<u16>,
    pub sol_payment_allocation: Option<u64>,
    pub spl_token_allocation: Option<u64>,
    pub role: String,
    pub is_active: bool,
    pub metadata_uri: Option<String>,
    pub bump: u8,
}