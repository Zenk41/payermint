use crate::errors::VaultError;
use crate::state::{MemberSplit, VaultAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TriggerPayout<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    pub owner: Signer<'info>,
    // Add more if payout interacts with member wallets
}

pub fn trigger_payout(ctx: Context<TriggerPayout>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;
    let schedule = vault.payout_schedule.as_ref().ok_or(VaultError::NoSchedule)?;
    // Payout condition check logic here
    // Transfer funds to each member proportionally
    Ok(())
}