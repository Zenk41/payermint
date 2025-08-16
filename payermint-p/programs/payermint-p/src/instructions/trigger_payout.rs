use crate::constants::BATCH_SEED;
use crate::constants::GLOBAL_CONFIG_SEED;
use crate::errors::ErrorVault;
use crate::state::{GlobalConfig, Member, PayrollBatch, VaultAccount};
use anchor_lang::prelude::*;
#[derive(Accounts)]
#[instruction(batch_id: u64)]
pub struct CreatePayrollBatch<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 4 + 1 + 1, // discriminator + vault + batch_id + timestamp + total_amount + service_fee + payout_count + finalized + bump
        seeds = [BATCH_SEED, vault_account.key().as_ref(), &batch_id.to_le_bytes()],
        bump
    )]
    pub payroll_batch: Account<'info, PayrollBatch>,

    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePayrollBatch<'info> {
    pub fn create(&mut self, batch_id: u64, total_amount: u64, bump: u8) -> Result<()> {
        let service_fee = (total_amount * self.global_config.default_fee_bps as u64) / 10000;

        self.payroll_batch.set_inner(PayrollBatch {
            vault: self.vault_account.key(),
            batch_id,
            timestamp: Clock::get()?.unix_timestamp,
            total_amount,
            service_fee,
            payout_count: 0,
            finalized: false,
            bump,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessSolPayout<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        mut,
        constraint = payroll_batch.vault == vault_account.key(),
        constraint = !payroll_batch.finalized @ ErrorVault::BatchAlreadyFinalized
    )]
    pub payroll_batch: Account<'info, PayrollBatch>,

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

impl<'info> ProcessSolPayout<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        // Calculate service fee
        let service_fee = (amount * self.global_config.default_fee_bps as u64) / 10000;
        let net_amount = amount - service_fee;

        // Ensure vault has sufficient balance
        require!(
            self.vault_account.total_balance >= amount,
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

        // Update vault and batch state
        self.vault_account.total_balance -= amount;
        self.payroll_batch.payout_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FinalizePayrollBatch<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        mut,
        constraint = payroll_batch.vault == vault_account.key(),
        constraint = !payroll_batch.finalized @ ErrorVault::BatchAlreadyFinalized
    )]
    pub payroll_batch: Account<'info, PayrollBatch>,

    pub owner: Signer<'info>,
}

impl<'info> FinalizePayrollBatch<'info> {
    pub fn finalize(&mut self) -> Result<()> {
        self.payroll_batch.finalized = true;
        Ok(())
    }
}
