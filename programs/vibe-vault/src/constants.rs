use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const MINT: Pubkey = pubkey!("91AgzqSfXnCq6AJm5CPPHL3paB25difEJ1TfSnrFKrf");
// pub const LOCK_TIME: i64 = 86400 * 30;
// #[cfg(not(feature = "testing"))]
pub const LOCK_TIME: i64 = 1;