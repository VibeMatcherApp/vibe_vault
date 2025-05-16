use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Custom error message")]
    IlegalWithdraw,
}


#[macro_export]
macro_rules! check_condition {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err(error!(ErrorCode::$error));
        }
    };
}