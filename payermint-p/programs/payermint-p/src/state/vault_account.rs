use anchor_spl::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum VaultType {
    Company,
    Organization,
    Individuals,
    Divisions,
}

#[account]
#[derive(InitSpace)]
pub struct VaultAccount {
    pub owner: Pubkey,
    pub vault_type: VaultType,
    pub members: Vec<MemberSplit>,
    pub payout_schedule: Option<PayoutSchedule>,
    pub total_balance: u64,
    pub last_deposit_ts: i64,
    pub metadata_uri: Option<String>,
    pub code_claim: String,
}
