use anchor_lang::prelude::*;

use crate::check_condition;
use crate::error::ErrorCode;
use crate::UserState;
use crate::VaultState;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
#[derive(Accounts)]
pub struct Unstake<'info> {
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
        seeds=[VaultState::SEEDS, mint.key().as_ref()],
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

impl<'info> Unstake<'info> {
    pub fn validate(&mut self, withdraw: u64) -> Result<()> {
        let current_stake_amount = self.user_state.staked_amount;
        check_condition!(current_stake_amount >= withdraw, IlegalWithdraw);
        check_condition!(self.user_state.staked_at + self.user_state.lock_time < Clock::get()?.unix_timestamp, CustomError);
        Ok(())
    }

    pub fn withdraw(&mut self, withdraw: u64) -> Result<()> {
        self.validate(withdraw)?;
        let signer_seeds: [&[&[u8]]; 1] = [&[
            VaultState::SEEDS,
            self.mint.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]]];

        let xfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.vault_state.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            xfer_accounts,
            &signer_seeds,
        );

        transfer_checked(ctx, withdraw, self.mint.decimals)?;

        Ok(())
    }
}
