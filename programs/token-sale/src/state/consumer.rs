use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Consumer {
    pub token_purchased: u64,
}

impl Default for Consumer {
    fn default() -> Self {
        Self { token_purchased: 0 }
    }
}

impl Consumer {
    pub fn buy_limit_exceeded(&mut self, amount: u64, limit: u64) -> bool {
        if self.token_purchased + amount > limit {
            return true;
        }
        false
    }
}
