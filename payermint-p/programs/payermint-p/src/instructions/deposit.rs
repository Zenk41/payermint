use crate::errors::VaultError;
use crate::state::{AssetType, MemberSplit, VaultAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    pub owner: Signer<'info>,
    // Optionally add token accounts and mints for SPL if needed
}

pub fn deposit(ctx: Context<Deposit>, asset: AssetType, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;
    // Token/SOL transfer logic here (CPI for SPL, lamports for SOL)
    match asset {
        AssetType::SOL => {
            vault.total_balance = vault.total_balance.checked_add(amount).unwrap();
        }
        AssetType::SPLToken { mint: _ } => {
            // handled in CPI layer
        }
    }
    vault.last_deposit_ts = clock.unix_timestamp;
    Ok(())
}
