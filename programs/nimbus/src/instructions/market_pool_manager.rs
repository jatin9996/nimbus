use anchor_lang::prelude::*;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke_signed,
    program_error::ProgramError,
    spl_token::instruction::transfer,
    system_instruction,
};
use std::collections::HashMap;
use crate::instructions::MarketRegistry;
use crate::config::{AUTHORIZED_ADMINS, ADMIN1_PUBKEY, ADMIN2_PUBKEY};

#[derive(Accounts)]
pub struct MarketPool<'info> {
    #[account(mut)]
    pub pool_account: Account<'info, Pool>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, spl_token::id>,
    #[account(init, payer = user, space = 8 + 256)]
    pub new_pool_account: Account<'info, Pool>,
    pub admin: Signer<'info>,
    pub market_registry: Account<'info, MarketRegistry>,
}

pub struct Pool {
    pub market_id: Pubkey,
    pub market_pool_id: Pubkey,
    pub total_deposits: u64,
    pub is_active: bool,
    pub deposits: HashMap<Pubkey, u64>,
    pub admin: Pubkey,
}

#[event]
pub struct MarketPoolCreated {
    pub market_id: Pubkey,
    pub admin: Pubkey,
}

#[event]
pub struct DepositMade {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawalMade {
    pub user: Pubkey,
    pub amount: u64,
}

// create marketpool id //
impl Pool {
    pub fn create_market_pool(ctx: Context<MarketPool>, market_id: Pubkey) -> Result<Pubkey, ProgramError> {
        let registry = &mut ctx.accounts.market_registry;
        if registry.active_markets.contains(&market_id) {
            msg!("Marketpool ID already exists: {}", market_id);
            return Err(ProgramError::Custom(201)); 
        }
        registry.add_market(market_id)?;
        let pool = &mut ctx.accounts.new_pool_account;
        pool.market_id = market_id;
        pool.market_pool_id = Pubkey::new_unique();
        pool.total_deposits = 0;
        pool.is_active = true;
        pool.deposits = HashMap::new();
        if pool.market_id != market_id || !pool.is_active {
            msg!("Failed to correctly initialize the market pool");
            return Err(ProgramError::InvalidAccountData);
        }
        emit!(MarketPoolCreated {
            market_id,
            admin: ctx.accounts.admin.key(),
        });
        Ok(pool.market_pool_id)
    }

    pub fn deposit(ctx: Context<MarketPool>, user: Pubkey, amount: u64) -> ProgramResult {
        let pool = &mut ctx.accounts.pool_account;
        if !pool.is_active {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if the user is one of the admins
        if !AUTHORIZED_ADMINS.contains(&ctx.accounts.admin.key()) {
            msg!("Unauthorized access by {}", ctx.accounts.admin.key());
            return Err(ProgramError::Unauthorized);
        }

        pool.total_deposits += amount;
        *pool.deposits.entry(user).or_insert(0) += amount;
        emit!(DepositMade { user, amount });
        Ok(())
    }

    pub fn withdraw(ctx: Context<MarketPool>, user: Pubkey, amount: u64) -> ProgramResult {
        let pool = &mut ctx.accounts.pool_account;
        if !pool.is_active {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if the user is one of the admins
        if !AUTHORIZED_ADMINS.contains(&ctx.accounts.admin.key()) {
            msg!("Unauthorized access by {}", ctx.accounts.admin.key());
            return Err(ProgramError::Unauthorized);
        }

        if pool.deposits.get(&user).unwrap_or(&0) < &amount {
            msg!("Insufficient funds for withdrawal by {}", user);
            return Err(ProgramError::InsufficientFunds);
        }

        *pool.deposits.get_mut(&user).unwrap() -= amount;
        emit!(WithdrawalMade { user, amount });
        Ok(())
    }

    pub fn cancel_market(ctx: Context<MarketPool>) -> ProgramResult {
        let admin = &ctx.accounts.admin;

        // Verify that the admin is authorized
        if !AUTHORIZED_ADMINS.contains(&admin.key) {
            msg!("Unauthorized attempt to cancel market by {}", admin.key);
            return Err(ProgramError::Unauthorized);
        }

        let registry = &mut ctx.accounts.market_registry;
        registry.remove_market(ctx.accounts.pool_account.market_id)?;

        let pool = &mut ctx.accounts.pool_account;
        pool.is_active = false;

        msg!("Market successfully cancelled by admin {}", admin.key);
        Ok(())
    }

    pub fn select_winners(ctx: Context<MarketPool>, vrf_account: AccountInfo) -> ProgramResult {
        // Temporarily disable randomness functionality
        /*
        let randomness_ctx = Context::new(
            ctx.program_id,
            RequestRandomness {
                vrf_account: vrf_account.into_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
            ctx.remaining_accounts,
        );

        let random_number = request_vrf_randomness(randomness_ctx)?;
        // Logic to select winners based on `random_number`
        */
        msg!("Randomness functionality temporarily disabled.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use std::collections::HashSet;

    struct MockMarketRegistry {
        active_markets: HashSet<Pubkey>,
    }

    impl MockMarketRegistry {
        fn new() -> Self {
            MockMarketRegistry {
                active_markets: HashSet::new(),
            }
        }

        fn add_market(&mut self, market_id: Pubkey) -> ProgramResult {
            if self.active_markets.contains(&market_id) {
                Err(ProgramError::Custom(201))
            } else {
                self.active_markets.insert(market_id);
                Ok(())
            }
        }
    }

    #[test]
    fn test_create_market_pool() {
        let mut pool = Pool {
            market_id: Pubkey::new_unique(),
            total_deposits: 0,
            is_active: false,
            deposits: HashMap::new(),
            admin: Pubkey::new_unique(),
        };
        let mock_context = Context::new(); // Mocked context

        assert_eq!(
            Pool::create_market_pool(mock_context, pool.market_id),
            Ok(())
        );
    }

    #[test]
    fn test_select_winners_disabled() {
        // Test to ensure the function handles being disabled correctly
        assert_eq!(
            Pool::select_winners(Context::default(), AccountInfo::default()),
            Ok(())
        );
    }
}

#[program]
mod market_pool_manager {
    use super::*;

    pub fn initialize_market_pool(ctx: Context<MarketPool>, market_id: Pubkey) -> ProgramResult {
        Pool::create_market_pool(ctx, market_id)
    }
}
