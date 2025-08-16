use anchor_lang::prelude::*;

use crate::errors::ErrorVault;
use crate::state::{GlobalConfig, Member, PayrollBatch, VaultAccount};
use crate::{MemberData, PayoutData};

use crate::constants::GLOBAL_CONFIG_SEED;

#[derive(Accounts)]
pub struct BulkAddMembers<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> BulkAddMembers<'info> {
    pub fn bulk_add(&mut self, members_data: Vec<MemberData>) -> Result<()> {
        // Validate total allocation doesn't exceed 100%
        let total_allocation: u32 = members_data
            .iter()
            .map(|m| m.allocation_bps.unwrap_or(0) as u32)
            .sum();

        require!(
            total_allocation <= 10000,
            ErrorVault::TotalAllocationExceeded
        );

        for member_data in members_data {
            require!(member_data.role.len() <= 16, ErrorVault::RoleTooLong);
            if let Some(ref uri) = member_data.metadata_uri {
                require!(uri.len() <= 200, ErrorVault::MetadataUriTooLong);
            }
            if let Some(bps) = member_data.allocation_bps {
                require!(bps <= 10000, ErrorVault::InvalidAllocationBps);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BulkProcessPayouts<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        mut,
        constraint = payroll_batch.vault == vault_account.key(),
        constraint = !payroll_batch.finalized @ ErrorVault::BatchAlreadyFinalized
    )]
    pub payroll_batch: Account<'info, PayrollBatch>,

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

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> BulkProcessPayouts<'info> {
    pub fn bulk_process(&mut self, payout_data: Vec<PayoutData>) -> Result<()> {
        let mut total_amount = 0u64;
        let mut total_service_fee = 0u64;

        // Calculate totals first
        for payout in &payout_data {
            let service_fee = (payout.amount * self.global_config.default_fee_bps as u64) / 10000;
            total_amount += payout.amount;
            total_service_fee += service_fee;
        }

        // Ensure vault has sufficient balance
        require!(
            self.vault_account.total_balance >= total_amount,
            ErrorVault::InsufficientVaultBalance
        );

        // Transfer total service fee to treasury
        if total_service_fee > 0 {
            **self
                .vault_account
                .to_account_info()
                .try_borrow_mut_lamports()? -= total_service_fee;
            **self.treasury.try_borrow_mut_lamports()? += total_service_fee;
        }

        // Update vault and batch state
        self.vault_account.total_balance -= total_amount;
        self.payroll_batch.payout_count += payout_data.len() as u32;

        Ok(())
    }
}
