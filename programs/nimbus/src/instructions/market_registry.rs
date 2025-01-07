use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use solana_program::entrypoint::ProgramResult;

#[account]
pub struct MarketRegistry {
    pub active_markets: Vec<Pubkey>,
}

impl MarketRegistry {
    pub fn add_market(&mut self, market_id: Pubkey) -> ProgramResult {
        if self.active_markets.contains(&market_id) {
            return Err(ProgramError::Custom(0)); // Market ID already exists
        }
        self.active_markets.push(market_id);
        Ok(())
    }

    pub fn remove_market(&mut self, market_id: Pubkey) -> ProgramResult {
        self.active_markets.retain(|&x| x != market_id);
        Ok(())
    }
}

