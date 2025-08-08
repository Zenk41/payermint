use crate::errors::VaultError;
use crate::state::{MemberSplit, VaultAccount, AssetType};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddWhitelistedAsset<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    pub owner: Signer<'info>,
}

// 8. Add Whitelisted Asset
pub fn add_whitelisted_asset(ctx: Context<AddWhitelistedAsset>, asset: AssetType) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(vault.whitelisted_assets.len() < 3, VaultError::TooManyAssets);
    let already = vault.whitelisted_assets.iter().any(|a| a == &asset);
    require!(!already, VaultError::AssetAlreadyWhitelisted);
    vault.whitelisted_assets.push(asset);
    Ok(())
}