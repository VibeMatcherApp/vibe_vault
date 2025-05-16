pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AsRo4gj9W5MW39xRmsToGkjMKesfwujQg97H7KuyVCcM");

#[program]
pub mod vibe_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>, receive: u64) -> Result<()> {
        ctx.accounts.save_vault(receive, &ctx.bumps)?;
        ctx.accounts.deposit(receive)
    }
    pub fn unstake(ctx: Context<Unstake>, withdraw: u64) -> Result<()> {
        ctx.accounts.withdraw(withdraw)
    }
}
