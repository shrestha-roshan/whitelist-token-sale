use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::{error::AuctionError, state::auction::Auction, InitAuctionParams, Whitelist, AUCTION, AUCTION_VAULT, WHITELIST};



#[derive(Accounts)]
#[instruction(params: InitAuctionArgs)]
pub struct InitAuction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,   
        payer = owner,
        space = 8 + Auction::INIT_SPACE,
        seeds = [AUCTION.as_bytes(), params.name.as_bytes()],
        bump
    )]
    pub auction: Box<Account<'info, Auction>>,
    #[account(
        init,
        payer = owner,
        space = 0,
        seeds = [AUCTION_VAULT.as_bytes(), auction.key().as_ref()],
        bump,
    )]
    /// CHECK: seeds has been checked
    pub auction_vault: AccountInfo<'info>,
    #[account(
        init,   
        payer = owner,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [WHITELIST.as_bytes(), auction.key().as_ref()],
        bump
    )]
    pub auciton_whitelist:Box<Account<'info, Whitelist>>,
    pub auction_token: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitAuctionArgs {
    pub name: String,
    pub start_time: i64,
    pub end_time: i64,
    pub tokens_in_pool: u64,            // pool of total tokens
    pub token_quantity_per_ticket: u64, // no. of tokens in one ticket
}

pub fn handler(ctx: Context<InitAuction>, params: InitAuctionArgs) -> Result<()> {
    let auction = &mut ctx.accounts.auction;

    // Ensure auction end time is greater than auction start time
    if params.start_time >= params.end_time {
        return Err(AuctionError::AuctionEnded.into());
    }

    auction.init(InitAuctionParams {
        auction_token: ctx.accounts.auction_token.key(),
        owner: ctx.accounts.owner.key(),
        name: params.name,
        start_time: params.start_time,
        end_time: params.end_time,
        tokens_in_pool: params.tokens_in_pool,
        token_quantity_per_ticket: params.token_quantity_per_ticket,
    });

    Ok(())
}