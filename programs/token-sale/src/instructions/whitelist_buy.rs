use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount, Transfer}};
use anchor_lang::system_program::{transfer as sol_transfer, Transfer as SolTransfer};

use crate::{ error::AuctionError, Auction, Consumer, Whitelist, AUCTION, AUCTION_VAULT, CONSUMER, WHITELIST};

#[derive(Accounts)]
pub struct WhitelistBuy<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [CONSUMER.as_bytes(), user.key().as_ref(), auction.key().as_ref()],
        space = 8 + Consumer::INIT_SPACE,
        bump
    )]
    user_pda: Box<Account<'info, Consumer>>,
    #[account(
        constraint = auction.token_details.auction_token == auction_token.key()
    )]
    auction_token: Account<'info, Mint>,
    #[account(
        mut,
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
    auction_whitelist: Box<Account<'info, Whitelist>>,
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
        init_if_needed,
        payer = user,
        associated_token::mint = auction_token,
        associated_token::authority = user,
    )]
    user_token_account: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>
}

pub fn handler(ctx: Context<WhitelistBuy>, amount: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let auction_vault_ata = &mut ctx.accounts.auction_vault_ata;
    let auction_vault = &mut ctx.accounts.auction_vault;
    let user_token_account = &mut ctx.accounts.user_token_account;
    let buyer_pda = &mut ctx.accounts.user_pda;
    let token_program = &ctx.accounts.token_program;
    let auction_whitelist = &ctx.accounts.auction_whitelist;
    let auction_token = &ctx.accounts.auction_token;
    let auction_token_decimals = auction_token.decimals;
    
    let remaining_tokens = auction.token_details.tokens_in_pool;
    let limit = auction.token_details.purchase_limit;
    let now = Clock::get()?.unix_timestamp;

    // Ensure auction has not ended
    require!(!auction.is_ended(now) && auction.is_started(now), AuctionError::InvalidAuctionTime);

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
    let auction_key = auction.key();    
    let signer_seed:&[&[&[_]]] = &[&[AUCTION_VAULT.as_bytes(), auction_key.as_ref(), &[ctx.bumps.auction_vault]]];
    let ctx_transfer = CpiContext::new_with_signer(token_program.to_account_info(), transfer_accounts, signer_seed);
    token::transfer(ctx_transfer, amount)?;

    // Transfer sol to vault
    let transfer_sol_accounts = SolTransfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.auction_vault.to_account_info(),
    };
    let ctx_transfer_sol = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_sol_accounts);
    let sol_price_per_token = auction.token_details.price_per_token;
    let sol_amount = amount/ 10u64.pow(auction_token_decimals as u32) * sol_price_per_token;
    sol_transfer(ctx_transfer_sol, sol_amount)?;

    buyer_pda.token_purchased += amount;
    auction.token_details.tokens_in_pool -= amount;
    auction.sol_accumulated += sol_amount;
    Ok(())
}
