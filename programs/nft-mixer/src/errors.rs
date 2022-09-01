use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Your balance is empty. Please deposit a NFT to mix n withdraw.")]
    EmptyBalance,
    #[msg("The pool is empty.")]
    EmptyPool,
    #[msg("Account Invalid")]
    AccountInvalid,
    #[msg("Invalid Mint")]
    InvalidMint,
}