use anchor_lang::prelude::*;
use crate::constants::GLOBAL_CONFIG_SEED;
use crate::errors::ErrorVault;
use crate::state::GlobalConfig;

#[derive(Accounts)]
pub struct InitializeGlobalConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 2 + 8 + 1, // discriminator + owner + treasury + fee_bps + next_company_id + bump
        seeds = [GLOBAL_CONFIG_SEED],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    /// CHECK: Treasury account for collecting fees
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGlobalConfig<'info> {
    pub fn initialize(&mut self, default_fee_bps: u16, bump: u8) -> Result<()> {
        self.global_config.set_inner(GlobalConfig {
            owner: self.owner.key(),
            treasury: self.treasury.key(),
            default_fee_bps,
            next_company_id: 1,
            bump,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateGlobalConfig<'info> {
    #[account(mut, has_one = owner)]
    pub global_config: Account<'info, GlobalConfig>,

    pub owner: Signer<'info>,
}

impl<'info> UpdateGlobalConfig<'info> {
    pub fn update_treasury(&mut self, new_treasury: Pubkey) -> Result<()> {
        self.global_config.treasury = new_treasury;
        Ok(())
    }

    pub fn update_default_fee(&mut self, new_fee_bps: u16) -> Result<()> {
        require!(new_fee_bps <= 10000, ErrorVault::InvalidFeeBps);
        self.global_config.default_fee_bps = new_fee_bps;
        Ok(())
    }
}
