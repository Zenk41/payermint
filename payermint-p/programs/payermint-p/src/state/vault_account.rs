use anchor_lang::prelude::*;

use crate::{MemberSplit, PayoutSchedule};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum VaultType {
    Company,
    Organization,
    Individuals,
    Divisions,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AssetType {
    SOL,
    SPLToken { mint: Pubkey },
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AllocationType {
    AllocationPerBps,
    AllocationSpecify,
}

#[account]
#[derive(InitSpace)]
pub struct VaultAccount {
    pub owner: Pubkey,
    #[max_len(32)]
    pub name: String,
    pub vault_type: VaultType,
    #[max_len(3)]
    pub whitelisted_assets: Vec<AssetType>,
    #[max_len(10)]
    pub members: Vec<MemberSplit>,
    pub payout_schedule: Option<PayoutSchedule>,
    pub total_balance: u64, // this is only fir sol
    pub required_balance: u64,
    pub last_deposit_ts: i64,
    pub allocation_type: AllocationType,
    #[max_len(50)]
    pub metadata_uri: Option<String>,
    #[max_len(10)]
    pub code_claim: Option<String>,
}
