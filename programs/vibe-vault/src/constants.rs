use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const MINT: Pubkey = pubkey!("FGpefZDXYQHb3JB2UZQm8E17V1niuvNdKBzAFDK5JrCw");
pub const LOCK_TIME: i64 = 86400 * 30;