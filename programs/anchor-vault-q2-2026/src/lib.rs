pub mod state;
use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;

pub use constants::*;
pub use instructions::*;
pub use state::*;
declare_id!("9w81bqMiXJh8Xg1DVj3fTENPe1hGgwTwAap4NnhtvgDt");

#[program]
pub mod anchor_vault_q2_2026 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
    
    
    
    
    // deposit funds
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
    // withdraw funds 
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
    // close
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
