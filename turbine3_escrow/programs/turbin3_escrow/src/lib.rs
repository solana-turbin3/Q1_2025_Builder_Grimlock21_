use anchor_lang::prelude::*;

mod contexts;
use contexts::*;

mod state;
use state::*;

declare_id!("HthgPvaR3PsW1yTHXgD38f4SMhTTATbPf2jJ3c9xX9nn");

#[program]
pub mod turbin3_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
