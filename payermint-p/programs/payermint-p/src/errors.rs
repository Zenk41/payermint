use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorVault {
    #[msg("Invalid fee basis points (must be <= 10000)")]
    InvalidFeeBps,

    #[msg("Name is too long (max 32 characters)")]
    NameTooLong,

    #[msg("Metadata URI is too long (max 200 characters)")]
    MetadataUriTooLong,

    // #[msg("Code claim is too long (max 10 characters)")]
    // CodeClaimTooLong,

    #[msg("Role is too long (max 16 characters)")]
    RoleTooLong,

    #[msg("Invalid allocation basis points (must be <= 10000)")]
    InvalidAllocationBps,

    #[msg("Asset is not whitelisted for this vault")]
    AssetNotWhitelisted,

    #[msg("Insufficient vault balance for payout")]
    InsufficientVaultBalance,

    #[msg("Member is not active")]
    MemberNotActive,

    #[msg("Payroll batch is already finalized")]
    BatchAlreadyFinalized,

    #[msg("Invalid treasury account")]
    InvalidTreasury,

    #[msg("Invalid member wallet")]
    InvalidMemberWallet,

    #[msg("Payout schedule is not active")]
    PayoutScheduleNotActive,

    #[msg("Next payout time has not been reached")]
    PayoutTimeNotReached,

    #[msg("Total allocation exceeds 100%")]
    TotalAllocationExceeded,

    // #[msg("Member already exists")]
    // MemberAlreadyExists,

    // #[msg("Invalid vault type")]
    // InvalidVaultType,

    // #[msg("Unauthorized access")]
    // Unauthorized,
}
