use anchor_lang::prelude::*;

use crate::{Auction, Whitelist, WHITELIST};

#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
    #[account(
        mut,
        seeds = [WHITELIST.as_bytes(), auction.key().as_ref()],
        bump
    )]
    pub auciton_whitelist: Account<'info, Whitelist>,
    pub admin: Signer<'info>,
    pub auction: Box<Account<'info, Auction>>,
}

pub fn handler(ctx: Context<RemoveWhitelist>, users: Vec<Pubkey>) -> Result<()> {
    let whitelist = &mut ctx.accounts.auciton_whitelist;
    whitelist.remove_addressess(users);
    Ok(())
}
