use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    #[max_len(200)]
    pub whitelist_addresses: Vec<Pubkey>,
}

impl Default for Whitelist {
    fn default() -> Self {
        Self {
            whitelist_addresses: Vec::new(),
        }
    }
}

impl Whitelist {
    pub fn add_addressess(&mut self, users: Vec<Pubkey>) {
        for user in &users {
            if self.whitelist_addresses.contains(user) {
                continue;
            } else {
                self.whitelist_addresses.push(*user);
            }
        }
    }

    pub fn remove_addressess(&mut self, address: Vec<Pubkey>) {
        if self.whitelist_addresses.is_empty() {
            return;
        }
        self.whitelist_addresses.retain(|x| !address.contains(x));
    }

    pub fn is_whitelisted(&self, address: &Pubkey) -> bool {
        self.whitelist_addresses.contains(address)
    }
}
