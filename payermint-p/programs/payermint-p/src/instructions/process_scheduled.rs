use anchor_lang::prelude::*;
use crate::constants::GLOBAL_CONFIG_SEED;
use crate::{ErrorVault, GlobalConfig, Member, VaultAccount};

#[derive(Accounts)]
pub struct ProcessScheduledPayout<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        constraint = member.vault == vault_account.key(),
        constraint = member.is_active @ ErrorVault::MemberNotActive
    )]
    pub member: Account<'info, Member>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// CHECK: Treasury account for fee collection
    #[account(
        mut,
        constraint = treasury.key() == global_config.treasury @ ErrorVault::InvalidTreasury
    )]
    pub treasury: AccountInfo<'info>,

    /// CHECK: Member wallet to receive payment
    #[account(
        mut,
        constraint = member_wallet.key() == member.wallet @ ErrorVault::InvalidMemberWallet
    )]
    pub member_wallet: AccountInfo<'info>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProcessScheduledPayout<'info> {
    pub fn process_scheduled_payout(&mut self) -> Result<()> {
        // Clone payout schedule so we don't hold an immutable borrow
        let payout_schedule = self
            .vault_account
            .payout_schedule
            .clone()
            .ok_or(ErrorVault::PayoutScheduleNotActive)?;

        require!(payout_schedule.active, ErrorVault::PayoutScheduleNotActive);

        // Check if it's time for the next payout
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= payout_schedule.next_payout_ts,
            ErrorVault::PayoutTimeNotReached
        );

        // Calculate payout amount based on member allocation
        let payout_amount = if let Some(allocation_bps) = self.member.allocation_bps {
            (self.vault_account.total_balance * allocation_bps as u64) / 10000
        } else if let Some(fixed_amount) = self.member.sol_payment_allocation {
            fixed_amount
        } else {
            return Err(ErrorVault::InvalidAllocationBps.into());
        };

        // Calculate service fee
        let service_fee = (payout_amount * self.global_config.default_fee_bps as u64) / 10000;
        let net_amount = payout_amount - service_fee;

        // Ensure vault has sufficient balance
        require!(
            self.vault_account.total_balance >= payout_amount,
            ErrorVault::InsufficientVaultBalance
        );

        // Transfer service fee to treasury
        if service_fee > 0 {
            **self
                .vault_account
                .to_account_info()
                .try_borrow_mut_lamports()? -= service_fee;
            **self.treasury.try_borrow_mut_lamports()? += service_fee;
        }

        // Transfer net amount to member
        **self
            .vault_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= net_amount;
        **self.member_wallet.try_borrow_mut_lamports()? += net_amount;

        // Update vault balance
        self.vault_account.total_balance -= payout_amount;

        // Update next payout timestamp
        let mut updated_schedule = payout_schedule;
        updated_schedule.next_payout_ts += updated_schedule.interval;
        self.vault_account.payout_schedule = Some(updated_schedule);

        Ok(())
    }
}
