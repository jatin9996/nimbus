use super::*;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::collections::HashMap;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Pubkey, Context<MarketPool>) {
        let program_id = Pubkey::new_unique(); // Corrected line
        let market_id = Pubkey::new_unique();
        let context = Context::new(); // Mocked context
        (market_id, context)
    }

    #[test]
    fn test_create_market_pool() {
        let (market_id, context) = setup();
        assert_eq!(
            Pool::create_market_pool(context, market_id),
            Ok(())
        );
    }

    #[test]
    fn test_deposit() {
        let (market_id, mut context) = setup();
        let user = Pubkey::new_unique();
        let deposit_amount = 1000u64;

        // Assuming market pool is already created and active
        assert_eq!(
            Pool::deposit(context, user, deposit_amount),
            Ok(())
        );
    }

    #[test]
    fn test_cancel_market() {
        let (_, context) = setup();
        assert_eq!(
            Pool::cancel_market(context),
            Ok(())
        );
    }

    #[test]
    fn test_select_winners() {
        let (_, context) = setup();
        let vrf_account = AccountInfo::new(
            &Pubkey::new_unique(),
            false,
            true,
            &mut 0,
            &mut vec![],
            &Pubkey::new_unique(),
            false,
            0,
        );

        assert_eq!(
            Pool::select_winners(context, vrf_account),
            Ok(())
        );
    }
}
