// // use crate::errors::VaultError;
// // use crate::state::{MemberSplit, VaultAccount};
// // use anchor_lang::prelude::*;


// // // #[derive(Accounts)]
// // // pub struct Claim<'info> {
// // //     #[account(mut)]
// // //     pub vault: Account<'info, VaultAccount>,
// // //     pub claimant: Signer<'info>,
// // //     
// // // }

// // // pub fn claim(ctx: Context<Claim>, code: String) -> Result<()> {
// // //     let vault = &mut ctx.accounts.vault;
// // //     require!(vault.code_claim == Some(code.clone()), VaultError::InvalidClaimCode);
// // //     // Check eligibility and transfer to claimant
// // //     vault.code_claim = None; // Reset after use
// // //     Ok(())
// // // }