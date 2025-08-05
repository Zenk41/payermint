use anchor_lang::prelude::*;

declare_id!("7UtSF3QDsZYECrkwfAURFjXvkbvCetvByi8itj5MSen6");

#[program]
pub mod payermint_p {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
