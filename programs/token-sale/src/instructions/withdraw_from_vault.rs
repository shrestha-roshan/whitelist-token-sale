use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::error::AuctionError;
use crate::{Auction, AUCTION, AUCTION_VAULT};

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(
        mut,
        seeds = [AUCTION.as_bytes(), auction.name.as_bytes()],
        constraint = auction.token_details.auction_token == auction_token.key(),
        constraint = auction.owner == *admin.key,
        bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [AUCTION_VAULT.as_bytes(), auction.key().as_ref()],
        bump
    )]
    /// CHECK: seeds has been checked
    pub auction_vault: AccountInfo<'info>,
    #[account(
        mut,
        constraint = auction_vault_ata.owner == auction_vault.key(),
        constraint = auction_vault_ata.mint == auction_token.key()
    )]
    pub auction_vault_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key(),
        constraint = admin_token_account.mint == auction_token.key()
    )]
    pub admin_token_account: Account<'info, TokenAccount>,
    pub auction_token: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawFromVault>) -> Result<()> {
    let admin = &mut ctx.accounts.admin;
    let auction = &mut ctx.accounts.auction;
    let auction_vault = &mut ctx.accounts.auction_vault;
    let auction_vault_ata = &mut ctx.accounts.auction_vault_ata;
    let token_program = &ctx.accounts.token_program;
    let admin_token_account = &mut ctx.accounts.admin_token_account;
    let auction_key = auction.key();

    let signer_seed: &[&[&[_]]] = &[&[
        AUCTION_VAULT.as_bytes(),
        auction_key.as_ref(),
        &[ctx.bumps.auction_vault],
    ]];

    require!(
        auction.is_ended(Clock::get()?.unix_timestamp),
        AuctionError::InvalidAuctionTime
    );

    if auction.token_details.tokens_in_pool > 0 {
        let transfer_accounts = Transfer {
            from: auction_vault_ata.to_account_info(),
            to: admin_token_account.to_account_info(),
            authority: auction_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            transfer_accounts,
            signer_seed,
        );
        transfer(cpi_ctx, auction.token_details.tokens_in_pool)?;
    }

    let accumulated_sol = auction.sol_accumulated;
    **auction_vault.try_borrow_mut_lamports()? = auction_vault
        .lamports()
        .checked_sub(accumulated_sol)
        .ok_or(ProgramError::InvalidArgument)?;

    **admin.try_borrow_mut_lamports()? = admin
        .lamports()
        .checked_add(accumulated_sol)
        .ok_or(ProgramError::InvalidArgument)?;

    auction.token_details.tokens_in_pool = 0;
    auction.sol_accumulated = 0;
    Ok(())
}
