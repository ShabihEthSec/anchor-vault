pub mod state;
use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;

// pub use constants::*;
pub use instructions::*;
pub use state::*;
declare_id!("24UpHQhbwrBNJCdVaSJ7HSBta3aAM2R9yn8yA4DUiRqu");

#[program]
pub mod anchor_vault_q2_2026 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
