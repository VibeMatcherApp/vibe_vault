use anchor_lang::prelude::*;

use crate::UserState;
use crate::VaultState;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::LOCK_TIME;
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[VaultState::SEEDS],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = vault_state,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [UserState::SEEDS, user.key().as_ref()],
        space = 8 + UserState::INIT_SPACE,
        bump
    )]
    pub user_state: Account<'info, UserState>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn save_vault(&mut self, receive: u64, bumps: &StakeBumps) -> Result<()> {
        self.user_state.set_inner(UserState {
            mint: self.mint.key(),
            staked_amount: receive,
            lock_time: LOCK_TIME,
            availible_claim: 0,
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.user_state,
        });
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.user_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint.decimals)
    }
}
