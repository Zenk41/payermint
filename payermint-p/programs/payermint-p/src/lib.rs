use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

pub mod errors;
pub use errors::*;

pub mod constants;
pub use constants::*;

pub mod helper;
pub use helper::*;

declare_id!("7UtSF3QDsZYECrkwfAURFjXvkbvCetvByi8itj5MSen6");

#[program]
pub mod payermint_p {
    // use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    use super::*;

    // GLOBAL CONFIG
    pub fn initialize_global_config(
        ctx: Context<InitializeGlobalConfig>,
        default_fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts
            .initialize(default_fee_bps, ctx.bumps.global_config)
    }

    pub fn update_treasury(ctx: Context<UpdateGlobalConfig>, new_treasury: Pubkey) -> Result<()> {
        ctx.accounts.update_treasury(new_treasury)
    }

    pub fn update_default_fee(ctx: Context<UpdateGlobalConfig>, new_fee_bps: u16) -> Result<()> {
        ctx.accounts.update_default_fee(new_fee_bps)
    }

    // VAULT MANAGEMENT
    pub fn create_vault(
        ctx: Context<CreateVault>,
        name: String,
        vault_type: VaultType,
        whitelisted_assets: Vec<AssetType>,
        payout_schedule: Option<PayoutSchedule>,
        allocation_type: AllocationType,
        metadata_uri: Option<String>,
        code_claim: Option<String>,
    ) -> Result<()> {
        ctx.accounts.create(
            name,
            vault_type,
            whitelisted_assets,
            payout_schedule,
            allocation_type,
            metadata_uri,
            code_claim,
            ctx.bumps.vault_account,
        )
    }

    pub fn update_payout_schedule(
        ctx: Context<UpdateVault>,
        schedule: Option<PayoutSchedule>,
    ) -> Result<()> {
        ctx.accounts.update_payout_schedule(schedule)
    }

    pub fn add_whitelisted_asset(ctx: Context<UpdateVault>, asset: AssetType) -> Result<()> {
        ctx.accounts.add_whitelisted_asset(asset)
    }

    pub fn remove_whitelisted_asset(ctx: Context<UpdateVault>, asset: AssetType) -> Result<()> {
        ctx.accounts.remove_whitelisted_asset(asset)
    }

    // MEMBER MANAGEMENT
    pub fn add_member(
        ctx: Context<AddMember>,
        role: String,
        allocation_bps: Option<u16>,
        sol_payment_allocation: Option<u64>,
        spl_token_allocation: Option<u64>,
        metadata_uri: Option<String>,
    ) -> Result<()> {
        ctx.accounts.add(
            role,
            allocation_bps,
            sol_payment_allocation,
            spl_token_allocation,
            metadata_uri,
            ctx.bumps.member,
        )
    }

    pub fn update_member_allocation(
        ctx: Context<UpdateMember>,
        allocation_bps: Option<u16>,
    ) -> Result<()> {
        ctx.accounts.update_allocation(allocation_bps)
    }

    pub fn update_member_payment_allocations(
        ctx: Context<UpdateMember>,
        sol_allocation: Option<u64>,
        spl_allocation: Option<u64>,
    ) -> Result<()> {
        ctx.accounts
            .update_payment_allocations(sol_allocation, spl_allocation)
    }

    pub fn toggle_member_active_status(ctx: Context<UpdateMember>) -> Result<()> {
        ctx.accounts.toggle_active_status()
    }

    pub fn remove_member(ctx: Context<RemoveMember>) -> Result<()> {
        // Account automatically closed due to close constraint
        Ok(())
    }

    // DEPOSITS
    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn deposit_spl_token(ctx: Context<DepositSplToken>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    // PAYROLL PROCESSING
    pub fn create_payroll_batch(
        ctx: Context<CreatePayrollBatch>,
        batch_id: u64,
        total_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .create(batch_id, total_amount, ctx.bumps.payroll_batch)
    }

    pub fn process_sol_payout(ctx: Context<ProcessSolPayout>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn finalize_payroll_batch(ctx: Context<FinalizePayrollBatch>) -> Result<()> {
        ctx.accounts.finalize()
    }

    // AUTOMATED PAYOUTS
    pub fn process_scheduled_payout(ctx: Context<ProcessScheduledPayout>) -> Result<()> {
        ctx.accounts.process_scheduled_payout()
    }

    // BULK OPERATIONS
    pub fn bulk_add_members(
        ctx: Context<BulkAddMembers>,
        members_data: Vec<MemberData>,
    ) -> Result<()> {
        ctx.accounts.bulk_add(members_data)
    }

    pub fn bulk_process_payouts(
        ctx: Context<BulkProcessPayouts>,
        payout_data: Vec<PayoutData>,
    ) -> Result<()> {
        ctx.accounts.bulk_process(payout_data)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
