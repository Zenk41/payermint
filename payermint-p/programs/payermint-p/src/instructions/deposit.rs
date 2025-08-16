use crate::errors::ErrorVault;
use crate::state::{AssetType, VaultAccount};

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> DepositSol<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // Verify SOL is whitelisted
        require!(
            self.vault_account
                .whitelisted_assets
                .contains(&AssetType::SOL),
            ErrorVault::AssetNotWhitelisted
        );

        // Transfer SOL from depositor to vault account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &self.depositor.key(),
            &self.vault_account.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                self.depositor.to_account_info(),
                self.vault_account.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        // Update vault balance
        self.vault_account.total_balance += amount;
        self.vault_account.last_deposit_ts = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositSplToken<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_account,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositSplToken<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // Verify token is whitelisted
        let asset = AssetType::SPLToken {
            mint: self.mint.key(),
        };
        require!(
            self.vault_account.whitelisted_assets.contains(&asset),
            ErrorVault::AssetNotWhitelisted
        );

        // Transfer tokens from depositor to vault
        let transfer_accounts = TransferChecked {
            from: self.depositor_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_token_account.to_account_info(),
            authority: self.depositor.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(ctx, amount, self.mint.decimals)?;

        // Update vault balance
        self.vault_account.last_deposit_ts = Clock::get()?.unix_timestamp;

        Ok(())
    }
}