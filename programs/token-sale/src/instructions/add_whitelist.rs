use anchor_lang::prelude::*;

use crate::{Auction, Whitelist, WHITELIST};

#[derive(Accounts)]
pub struct AddWhitelist<'info> {
    #[account(
        mut,
        seeds = [WHITELIST.as_bytes(), auction.key().as_ref()],
        bump
    )]
    pub auciton_whitelist: Account<'info, Whitelist>,
    pub admin: Signer<'info>,
    pub auction: Box<Account<'info, Auction>>,
}

pub fn handler(ctx: Context<AddWhitelist>, users: Vec<Pubkey>) -> Result<()> {
    let whitelist = &mut ctx.accounts.auciton_whitelist;
    whitelist.add_addressess(users);
    Ok(())
}
