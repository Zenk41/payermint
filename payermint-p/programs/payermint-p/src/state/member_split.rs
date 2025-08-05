use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MemberSplit {
    wallet: Pubkey,
    allocation_bps: u16,
    role: String,
    is_active: bool,
    metadata_uri: Option<String>,
}
