use anchor_lang::prelude::*;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{clock::Clock, Sysvar},
};
use std::collections::HashMap;

#[derive(Accounts)]
pub struct MarketClosure<'info> {
    #[account(mut)]
    pub market_pool: Account<'info, MarketPool>,
    pub clock: Sysvar<Clock>,
}

#[account]
pub struct MarketPool {
    pub winners: HashMap<Pubkey, u64>,
    pub closure_time: i64,
    pub total_prize: u64,
    pub claimed_prizes: HashMap<Pubkey, u64>,
}

impl MarketPool {
    pub fn close_market(&mut self, current_time: i64) -> ProgramResult {
        if current_time < self.closure_time {
            return Err(ProgramError::InvalidAccountData);
        }
        self.closure_time = current_time;
        Ok(())
    }

    pub fn claim_prize(ctx: Context<MarketClosure>, winner: Pubkey, claim_amount: u64) -> ProgramResult {
        let market_pool = &mut ctx.accounts.market_pool;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > market_pool.closure_time + 15 * 86400 {
            msg!("Claim period expired for winner {}", winner);
            return Err(ProgramError::InvalidAccountData);
        }

        if let Some(&amount) = market_pool.winners.get(&winner) {
            if claim_amount > amount {
                msg!("Invalid claim amount by winner {}", winner);
                return Err(ProgramError::InvalidArgument);
            }

            *market_pool.claimed_prizes.entry(winner).or_insert(0) += claim_amount;
            market_pool.total_prize -= claim_amount;
            msg!("Prize claimed by winner {}: {}", winner, claim_amount);
            Ok(())
        } else {
            msg!("No prize record found for winner {}", winner);
            Err(ProgramError::InvalidAccountData)
        }
    }
}
