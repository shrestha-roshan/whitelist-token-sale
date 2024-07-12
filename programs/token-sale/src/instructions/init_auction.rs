use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::{error::AuctionError, state::auction::Auction, InitAuctionParams, Whitelist, AUCTION, AUCTION_VAULT, WHITELIST};



#[derive(Accounts)]
#[instruction(args: InitAuctionArgs)]
pub struct InitAuction<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,   
        payer = admin,
        space = 8 + Auction::INIT_SPACE,
        seeds = [AUCTION.as_bytes(), args.name.as_bytes()],
        bump
    )]
    pub auction: Box<Account<'info, Auction>>,
    #[account(
        init,
        payer = admin,
        space = 0,
        seeds = [AUCTION_VAULT.as_bytes(), auction.key().as_ref()],
        bump,
    )]
    /// CHECK: seeds has been checked
    pub auction_vault: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = auction_token,
        associated_token::authority = auction_vault,
    )]
    pub auction_vault_ata:Account<'info, TokenAccount>,
    #[account(
        init,   
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [WHITELIST.as_bytes(), auction.key().as_ref()],
        bump
    )]
    pub auciton_whitelist:Box<Account<'info, Whitelist>>,
    pub auction_token: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitAuctionArgs {
    pub name: String,
    pub start_time: i64,
    pub end_time: i64,
    pub tokens_in_pool: u64,            // pool of total tokens
    pub purchase_limit: u64,
}

pub fn handler(ctx: Context<InitAuction>, args: InitAuctionArgs) -> Result<()> {
    let auction = &mut ctx.accounts.auction;

    // Ensure auction end time is greater than auction start time
    if args.start_time >= args.end_time {
        return Err(AuctionError::AuctionEnded.into());
    }

    auction.init(InitAuctionParams {
        auction_token: ctx.accounts.auction_token.key(),
        admin: ctx.accounts.admin.key(),
        name: args.name,
        start_time: args.start_time,
        end_time: args.end_time,
        tokens_in_pool: args.tokens_in_pool,
        purchase_limit: args.purchase_limit,
    });

    Ok(())
}