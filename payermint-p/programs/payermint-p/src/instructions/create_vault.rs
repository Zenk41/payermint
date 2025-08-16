use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::GLOBAL_CONFIG_SEED;
use crate::constants::VAULT_SEED;
use crate::errors::ErrorVault;
use crate::state::{AllocationType, GlobalConfig, PayoutSchedule};
use crate::state::{AssetType, VaultAccount, VaultType};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 32 + 4 + 10 * 33 + 1 + 20 + 8 + 8 + 8 + 8 + 1 + 1 + 204 + 1 + 14 + 1, // discriminator + owner (Pubkey) + treasury (Pubkey) + default_fee_bps (u16) + next_company_id + bump (u8)
        seeds = [VAULT_SEED, owner.key().as_ref(), &global_config.next_company_id.to_le_bytes()],
        bump
    )]
    pub vault_account: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateVault<'info> {
    pub fn create(
        &mut self,
        name: String,
        vault_type: VaultType,
        whitelisted_assets: Vec<AssetType>,
        payout_schedule: Option<PayoutSchedule>,
        allocation_type: AllocationType,
        metadata_uri: Option<String>,
        code_claim: Option<String>,
        bump: u8,
    ) -> Result<()> {
        require!(name.len() <= 32, ErrorVault::NameTooLong);
        if let Some(ref uri) = metadata_uri {
            require!(uri.len() <= 200, ErrorVault::MetadataUriTooLong);
        }
        if let Some(ref encrypted_code) = code_claim {
            // Encrypted code is passed from the client
            let encrypted_code_claim = encrypted_code.clone();

            self.vault_account.code_claim = Some(encrypted_code_claim);
        } else {
            // No code_claim provided
            self.vault_account.code_claim = None;
        }

        self.vault_account.set_inner(VaultAccount {
            owner: self.owner.key(),
            name,
            vault_type,
            whitelisted_assets,
            payout_schedule,
            total_balance: 0,        // Start with 0 total balance
            required_balance: 0,     // No required balance initially
            required_spl_balance: 0, // No required SPL token balance initially
            last_deposit_ts: Clock::get()?.unix_timestamp, // Current timestamp for the last deposit
            allocation_type,
            metadata_uri,
            code_claim: self.vault_account.code_claim.clone(),
            bump,
            spl_balances: Vec::new(), // Start with an empty list of SPL token balances
            sol_balance: 0,           // Start with 0 SOL balance
        });

        // Increment next company ID
        self.global_config.next_company_id += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateVault<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    pub owner: Signer<'info>,
}

impl<'info> UpdateVault<'info> {
    pub fn update_payout_schedule(&mut self, schedule: Option<PayoutSchedule>) -> Result<()> {
        self.vault_account.payout_schedule = schedule;
        Ok(())
    }

    pub fn add_whitelisted_asset(&mut self, asset: AssetType) -> Result<()> {
        if !self.vault_account.whitelisted_assets.contains(&asset) {
            self.vault_account.whitelisted_assets.push(asset);
        }
        Ok(())
    }

    pub fn remove_whitelisted_asset(&mut self, asset: AssetType) -> Result<()> {
        self.vault_account
            .whitelisted_assets
            .retain(|a| a != &asset);
        Ok(())
    }
}
