use anchor_lang::prelude::*;

declare_id!("4NxE8YEpNSigAhbGBpBBc43rHjEVDb82YPj4EHpSrBw2");

mod instructions;
pub mod state;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Make {}
