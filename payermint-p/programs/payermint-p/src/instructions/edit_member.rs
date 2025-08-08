use crate::errors::VaultError;
use crate::state::{MemberSplit, VaultAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditMember<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    pub owner: Signer<'info>,
}

pub fn edit_member(ctx: Context<EditMember>, wallet: Pubkey, updated: MemberSplit) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let index = vault.members.iter().position(|m| m.wallet == wallet);
    require!(index.is_some(), VaultError::MemberNotFound);
    let idx = index.unwrap();
    vault.members[idx] = updated;
    let total: u64 = vault.members.iter().map(|m| m.allocation_bps.unwrap_or(1) as u64).sum();
    require!(total <= 10_000, VaultError::AllocationExceeds100Percent);
    Ok(())
}
