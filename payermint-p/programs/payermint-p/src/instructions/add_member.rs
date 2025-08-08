use crate::errors::VaultError;
use crate::state::{AllocationType, MemberSplit, VaultAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,

    pub owner: Signer<'info>,
}

pub fn add_member(ctx: Context<AddMember>, new_member: MemberSplit) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let exists = vault.members.iter().any(|m| m.wallet == new_member.wallet);
    require!(!exists, VaultError::MemberAlreadyExists);

    match vault.allocation_type {
        AllocationType::AllocationPerBps => {
            let current_total: u64 = vault
                .members
                .iter()
                .map(|m| m.allocation_bps.unwrap_or(0) as u64)
                .sum();
            let new_total = current_total + new_member.allocation_bps.unwrap_or(0) as u64;
            require!(new_total <= 10_000, VaultError::AllocationExceeds100Percent);
        }
        AllocationType::AllocationSpecify => {
            let current_spl: u64 = vault
                .members
                .iter()
                .map(|m| m.spl_token_allocation.unwrap_or(0) as u64)
                .sum();
            let new_spl = current_spl + new_member.spl_token_allocation.unwrap_or(0) as u64;

            let current_sol: u64 = vault
                .members
                .iter()
                .map(|m| m.sol_payment_allocation.unwrap_or(0) as u64)
                .sum();
            let new_sol = current_sol + new_member.sol_payment_allocation.unwrap_or(0) as u64;

            require!(new_spl <= 10_000, VaultError::AllocationExceeds100Percent);
            require!(new_sol <= 10_000, VaultError::AllocationExceeds100Percent);
        }
    }

    vault.members.push(new_member);
    Ok(())
}
