use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::{VaultAccount,VaultType,AssetType};
use crate::constants::{VAULT_SEED,SPL_ACCOUNT_SEED };
use crate::errors::VaultError;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer, 
        space = 8 + VaultAccount::INIT_SPACE,
        seeds = [VAULT_SEED.as_bytes(),name.as_bytes()],
        bump
    )]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitSplTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Ensure that the signer is the vault owner
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,

    /// Must match `vault.owner`
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = payer,
        token::mint = spl_mint,
        token::authority = vault,
        seeds = [SPL_ACCOUNT_SEED.as_bytes(), vault.key().as_ref()],
        bump
    )]
    pub spl_token_account: Account<'info, TokenAccount>,

    pub spl_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_vault(
    ctx: Context<CreateVault>,
    name: String,
    vault_type: VaultType,
    whitelisted_assets: Vec<AssetType>,
    metadata_uri: Option<String>,
) -> Result<()> {
    require!(whitelisted_assets.len() <= 3, VaultError::TooManyAssets);
    let vault = &mut ctx.accounts.vault;

    vault.owner = ctx.accounts.payer.key();
    vault.name = name;
    vault.vault_type = vault_type;
    vault.whitelisted_assets = whitelisted_assets;
    vault.members = vec![];
    vault.payout_schedule = None;
    vault.total_balance = 0;
    vault.last_deposit_ts = Clock::get()?.unix_timestamp;
    vault.metadata_uri = metadata_uri;
    vault.code_claim = None;

    Ok(())
}


pub fn init_spl_token_account(ctx: Context<InitSplTokenAccount>) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let mint = ctx.accounts.spl_mint.key();

    // Check if the SPL token mint is whitelisted
    let is_whitelisted = vault
        .whitelisted_assets
        .iter()
        .any(|asset| match asset {
            AssetType::SPLToken { mint: whitelisted_mint } => whitelisted_mint == &mint,
            _ => false,
        });

    require!(is_whitelisted, VaultError::AssetNotWhitelisted);

    // Token account creation already handled by Anchor via init attribute
    Ok(())
}

