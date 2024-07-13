use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{Auction, AUCTION, AUCTION_VAULT};

#[derive(Accounts)]
pub struct DepositToVault<'info> {
    #[account(
        mut,
        seeds = [AUCTION.as_bytes(), auction.name.as_bytes()],
        constraint = auction.token_details.auction_token == auction_token.key(),
        bump
    )]
    pub auction: Account<'info, Auction>,
    pub depositor: Signer<'info>,
    #[account(
        mut,
        seeds = [AUCTION_VAULT.as_bytes(), auction.key().as_ref()],
        bump
    )]
    pub auction_vault: AccountInfo<'info>,
    #[account(
        mut,
        constraint = auction_vault_ata.owner == auction_vault.key(),
        constraint = auction_vault_ata.mint == auction_token.key()
    )]
    pub auction_vault_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = depositor_token_account.owner == depositor.key(),
        constraint = depositor_token_account.mint == auction_token.key()
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,
    pub auction_token: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let auction_vault_ata = &mut ctx.accounts.auction_vault_ata;
    let token_program = &ctx.accounts.token_program;
    let depositor_token_account = &ctx.accounts.depositor_token_account;

    let transfer_accounts = Transfer {
        from: depositor_token_account.to_account_info(),
        to: auction_vault_ata.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_accounts);
    transfer(cpi_ctx, amount)?;

    auction.token_details.tokens_in_pool += amount;
    Ok(())
}
