use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("2mmarH7Ut2eALnJy9pGofGHhabUYQNNqxUhtY34zUatX");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", state.key().as_ref()],
        bump = state.state_bump,
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.valut_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds: &[&[u8]; 3] = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.valut_bump],
        ];

        let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];

        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // we also have to assert that the amount left is greater than the rent exemption amount
        transfer(cpi_ctx, amount)
    }

}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.valut_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        self.state.amount = 0;
        Ok(())
    }
}

// to do, write a close vault function

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub valut_bump: u8, // so you don't have to recalculate on chain
    pub state_bump: u8,
    pub amount: u64,
}

