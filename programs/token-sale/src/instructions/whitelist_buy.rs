use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::{Auction, AUCTION, AUCTION_VAULT};

#[derive(Accounts)]
pub struct WhitelistBuy<'info> {
    user: Signer<'info>,
    auction_token: Account<'info, Mint>,
    #[account(
        seeds = [AUCTION.as_bytes(), auction.name.as_bytes()],
        constraint = auction.token_details.auction_token == auction_token.key(),
        bump        
    )]
    auction: Box<Account<'info, Auction>>,
    #[account(
        mut,
        seeds = [AUCTION_VAULT.as_bytes(), auction.key().as_ref()],
        bump
    )]
    auction_vault: AccountInfo<'info>,
    #[account(
        mut,
        constraint = auction_vault_ata.owner == auction_vault.key(),
        constraint = auction_vault_ata.mint == auction_token.key()
    )]
    auction_vault_ata: Account<'info, TokenAccount>,
    system_program: Program<'info, System>
}

pub fn handler(ctx: Context<WhitelistBuy>, amount: u64) -> Result<()> {
    
    Ok(())
}
