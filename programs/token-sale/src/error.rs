use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionError {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Auction has already ended")]
    AuctionEnded,
    #[msg("Auction time Invalid")]
    InvalidAuctionTime,
    #[msg("User is already in the whitelist")]
    UserAlreadyInWhitelist,
    #[msg("Purchase limit exceeded")]
    PurchaseLimitExceeded,
    #[msg("Insufficient tokens in pool")]
    InsufficientTokens,
    #[msg("User not whitelisted")]
    UserNotWhitelisted,
}
