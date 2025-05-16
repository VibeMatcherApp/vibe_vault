use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub mint: Pubkey,
    pub staked_amount: u64,
    pub lock_time: i64,
    pub availible_claim: u64,
    pub staked_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub mint: Pubkey,
    pub vault_bump: u8
}

impl VaultState {
    pub const SEEDS: &'static [u8] = b"vault";
}

impl UserState {
    pub const SEEDS: &'static [u8] = b"user_vault";
}
