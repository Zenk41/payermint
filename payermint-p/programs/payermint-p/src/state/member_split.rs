use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MemberSplit {
    pub wallet: Pubkey,
    pub allocation_bps: Option<u16>,
    pub sol_payment_allocation: Option<u16>,
    pub spl_token_allocation: Option<u16>,
    #[max_len(5)]
    pub role: String,
    pub is_active: bool,
    #[max_len(50)]
    pub metadata_uri: Option<String>,
}
