use crate::errors::VaultError;
use crate::state::{MemberSplit, PayoutSchedule, VaultAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ConfigureSchedule<'info> {
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, VaultAccount>,
    pub owner: Signer<'info>,
}

pub fn configure_payout_schedule(
    ctx: Context<ConfigureSchedule>,
    schedule: PayoutSchedule,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.payout_schedule = Some(schedule);
    Ok(())
}
