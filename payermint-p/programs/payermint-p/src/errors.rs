use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Empty name")]
    EmptyName,
    #[msg("The requested SPL token is not whitelisted for this vault.")]
    AssetNotWhitelisted,
    #[msg("Too many whitelisted assets.")]
    TooManyAssets,
    #[msg("Total allocation exceeds 10000 basis points.")]
    AllocationExceeds100Percent,
    #[msg("Member already exists")]
    MemberAlreadyExists,
    #[msg("Member not found")]
    MemberNotFound,
    #[msg("No Schedule")]
    NoSchedule,
    #[msg("Claim Code is invalid")]
    InvalidClaimCode,
    #[msg("Asset has been whitelisted before")]
    AssetAlreadyWhitelisted,
}
