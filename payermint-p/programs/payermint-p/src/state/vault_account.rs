use anchor_lang::prelude::*;

use crate::state::PayoutSchedule;

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
pub struct VaultAccount {
    pub owner: Pubkey,
    // #[max_len(32)]
    pub name: String,
    pub vault_type: VaultType,
    pub whitelisted_assets: Vec<AssetType>,
    pub payout_schedule: Option<PayoutSchedule>,
    pub total_balance: u64,
    pub required_balance: u64,
    pub required_spl_balance: u64,
    pub last_deposit_ts: i64,
    pub allocation_type: AllocationType,
    // #[max_len(200)]
    pub metadata_uri: Option<String>,
    // #[max_len(10)]
    pub code_claim: Option<String>, // since i didnt found the practical way to handle this iam gonna handle the code claim offchain
    pub bump: u8,

    pub spl_balances: Vec<SplTokenBalance>, // A list of balances for SPL tokens in the vault
    pub sol_balance: u64,                   // SOL balance in the vault (native Solana tokens)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SplTokenBalance {
    pub mint: Pubkey, // The mint address of the SPL token
    pub balance: u64, // The balance of the SPL token
}
