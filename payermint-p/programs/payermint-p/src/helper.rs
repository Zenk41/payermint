
use anchor_lang::prelude::*;

use crate::AssetType;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MemberData {
    pub wallet: Pubkey,
    pub role: String,
    pub allocation_bps: Option<u16>,
    pub sol_payment_allocation: Option<u64>,
    pub spl_token_allocation: Option<u64>,
    pub metadata_uri: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PayoutData {
    pub member: Pubkey,
    pub amount: u64,
    pub asset_type: AssetType,
}