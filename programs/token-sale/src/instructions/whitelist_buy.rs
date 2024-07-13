use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{error::AuctionError, Auction, Consumer, Whitelist, AUCTION, AUCTION_VAULT, CONSUMER, WHITELIST};

#[derive(Accounts)]
pub struct WhitelistBuy<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [CONSUMER.as_bytes(), auction.name.as_bytes()],
        space = 8 + Consumer::INIT_SPACE,
        bump
    )]
    user_pda: Account<'info, Consumer>,
    #[account(
        constraint = auction.token_details.auction_token == auction_token.key()
    )]
    auction_token: Account<'info, Mint>,
    #[account(
        seeds = [AUCTION.as_bytes(), auction.name.as_bytes()],
        constraint = auction.token_details.auction_token == auction_token.key(),
        bump        
    )]
    auction: Box<Account<'info, Auction>>,
    #[account(
        mut,
        seeds = [WHITELIST.as_bytes(), auction.key().as_ref()],
        bump
    )]
    auction_whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        seeds = [AUCTION_VAULT.as_bytes(), auction.key().as_ref()],
        bump
    )]
    /// CHECK: seeds has been checked
    auction_vault: AccountInfo<'info>,
    #[account(
        mut,
        constraint = auction_vault_ata.owner == auction_vault.key(),
        constraint = auction_vault_ata.mint == auction_token.key()
    )]
    auction_vault_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = auction_vault_ata.owner == user.key(),
        constraint = auction_vault_ata.mint == auction_token.key()
    )]
    user_token_account: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WhitelistBuy>, amount: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let auction_vault_ata = &mut ctx.accounts.auction_vault_ata;
    let auction_vault = &ctx.accounts.auction_vault;
    let user_token_account = &ctx.accounts.user_token_account;
    let buyer_pda = &mut ctx.accounts.user_pda;
    let token_program = &ctx.accounts.token_program;
    let auction_whitelist = &ctx.accounts.auction_whitelist;
    
    let remaining_tokens = auction.token_details.tokens_in_pool;
    let limit = auction.token_details.purchase_limit;

    // Ensure user is whitelisted
    require!(auction_whitelist.is_whitelisted(ctx.accounts.user.key), AuctionError::UserNotWhitelisted);

    // Ensure user has not exceeded purchase limit
    require!(buyer_pda.token_purchased + amount <= limit, AuctionError::PurchaseLimitExceeded);

    // Ensure user is not purchasing more tokens than are available
    require!(amount <= remaining_tokens, AuctionError::InsufficientTokens);

    let transfer_accounts = Transfer {
        from: auction_vault_ata.to_account_info(),
        to: user_token_account.to_account_info(),
        authority: auction_vault.to_account_info(),
    };
    let signer_seed:&[&[&[_]]] = &[&[AUCTION.as_bytes(), auction.name.as_bytes(), &[ctx.bumps.auction]]];
    let ctx_transfer = CpiContext::new_with_signer(token_program.to_account_info(), transfer_accounts, signer_seed);
    token::transfer(ctx_transfer, amount)?;

    buyer_pda.token_purchased += amount;
    auction.token_details.tokens_in_pool -= amount;
    Ok(())
}
