pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ANmXDQMMFJ7zgrSaRLEnYbxeHthDAHCkZTMJQw3gDrfP");

#[program]
pub mod token_sale {
    use super::*;

    pub fn init_auction(
        ctx: Context<InitAuction>,
        init_auction_args: InitAuctionArgs,
    ) -> Result<()> {
        init_auction::handler(ctx, init_auction_args)
    }

    pub fn deposit_to_vault(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
        deposit_to_vault::handler(ctx, amount)
    }

    pub fn add_whitelist(ctx: Context<AddWhitelist>, users: Vec<Pubkey>) -> Result<()> {
        add_whitelist::handler(ctx, users)
    }

    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, users: Vec<Pubkey>) -> Result<()> {
        remove_whitelist::handler(ctx, users)
    }

    pub fn whitelist_buy(ctx: Context<WhitelistBuy>, amount: u64) -> Result<()> {
        whitelist_buy::handler(ctx, amount)
    }
}
