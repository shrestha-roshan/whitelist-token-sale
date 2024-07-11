use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionError {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Auction has already ended")]
    AuctionEnded,
}
