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
}
