use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub owner: Pubkey,
    #[max_len(30)]
    pub name: String,
    pub start_time: i64,
    pub end_time: i64,
    pub token_details: TokenDetails,
}

#[account]
#[derive(InitSpace)]
pub struct TokenDetails {
    pub mint: Pubkey,
    pub tokens_in_pool: u64,
    pub remaining_tokens: u64,
    pub token_quantity_per_ticket: u64,
}

impl Default for TokenDetails {
    fn default() -> Self {
        Self {
            mint: Pubkey::default(),
            tokens_in_pool: 0,
            remaining_tokens: 0,
            token_quantity_per_ticket: 0,
        }
    }
}

impl Default for Auction {
    fn default() -> Self {
        Self {
            owner: Pubkey::default(),
            name: String::default(),
            start_time: 0,
            end_time: 0,
            token_details: TokenDetails::default(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitAuctionParams {
    pub auction_token: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub start_time: i64,
    pub end_time: i64,
    pub tokens_in_pool: u64,            // pool of total tokens
    pub token_quantity_per_ticket: u64, // no. of tokens in one ticket
}

impl Auction {
    pub fn init(&mut self, params: InitAuctionParams) {
        *self = Self::default();
        self.owner = params.owner;
        self.name = params.name;
        self.start_time = params.start_time;
        self.end_time = params.end_time;
        self.token_details.mint = params.auction_token;
        self.token_details.tokens_in_pool = params.tokens_in_pool;
        self.token_details.remaining_tokens = params.tokens_in_pool;
        self.token_details.token_quantity_per_ticket = params.token_quantity_per_ticket;
    }
}
