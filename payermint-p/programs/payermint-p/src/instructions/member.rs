use crate::constants::MEMBER_SEED;
use crate::errors::ErrorVault;
use crate::state::{Member, VaultAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(role: String)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    pub owner: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 2 + 8 + 8 + 4 + 16 + 1 + 1 + 204 + 1, // discriminator + vault + wallet + allocation_bps + sol_allocation + spl_allocation + role + is_active + metadata_uri + bump
        seeds = [MEMBER_SEED, vault_account.key().as_ref(), wallet.key().as_ref()],
        bump
    )]
    pub member: Account<'info, Member>,

    /// CHECK: The wallet address of the member
    pub wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddMember<'info> {
    pub fn add(
        &mut self,
        role: String,
        allocation_bps: Option<u16>,
        sol_payment_allocation: Option<u64>,
        spl_token_allocation: Option<u64>,
        metadata_uri: Option<String>,
        bump: u8,
    ) -> Result<()> {
        require!(role.len() <= 16, ErrorVault::RoleTooLong);
        if let Some(ref uri) = metadata_uri {
            require!(uri.len() <= 200, ErrorVault::MetadataUriTooLong);
        }
        if let Some(bps) = allocation_bps {
            require!(bps <= 10000, ErrorVault::InvalidAllocationBps);
        }

        self.member.set_inner(Member {
            vault: self.vault_account.key(),
            wallet: self.wallet.key(),
            allocation_bps,
            sol_payment_allocation,
            spl_token_allocation,
            role,
            is_active: true,
            metadata_uri,
            bump,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateMember<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        mut,
        constraint = member.vault == vault_account.key(),
        seeds = [MEMBER_SEED, vault_account.key().as_ref(), member.wallet.as_ref()],
        bump = member.bump
    )]
    pub member: Account<'info, Member>,

    pub owner: Signer<'info>,
}

impl<'info> UpdateMember<'info> {
    pub fn update_allocation(&mut self, allocation_bps: Option<u16>) -> Result<()> {
        if let Some(bps) = allocation_bps {
            require!(bps <= 10000, ErrorVault::InvalidAllocationBps);
        }
        self.member.allocation_bps = allocation_bps;
        Ok(())
    }

    pub fn update_payment_allocations(
        &mut self,
        sol_allocation: Option<u64>,
        spl_allocation: Option<u64>,
    ) -> Result<()> {
        self.member.sol_payment_allocation = sol_allocation;
        self.member.spl_token_allocation = spl_allocation;
        Ok(())
    }

    pub fn toggle_active_status(&mut self) -> Result<()> {
        self.member.is_active = !self.member.is_active;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RemoveMember<'info> {
    #[account(mut, has_one = owner)]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        mut,
        close = owner,
        constraint = member.vault == vault_account.key(),
        seeds = [MEMBER_SEED, vault_account.key().as_ref(), member.wallet.as_ref()],
        bump = member.bump
    )]
    pub member: Account<'info, Member>,

    #[account(mut)]
    pub owner: Signer<'info>,
}
