use super::*;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program_test::*;
use solana_sdk::account::Account;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_trade() {
        let program_id = Pubkey::from_str("Pubkey").unwrap();
        let mut accounts = Vec::new();
        let mut lamports = 0;
        let mut data = vec![0; 100];
        let account_info = AccountInfo::new(
            &Pubkey::new_unique(),
            true,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );
        accounts.push(account_info.clone()); // buyer
        accounts.push(account_info.clone()); // seller
        accounts.push(account_info); // token_program

        let trade_data = TradeData {
            buyer: Pubkey::new_unique(),
            seller: Pubkey::new_unique(),
            amount: 100,
            price: 10.0,
        };

        let signatures = vec![trade_data.buyer, trade_data.seller];

        assert_eq!(
            execute_trade(&accounts, &trade_data, &signatures),
            Ok(())
        );
    }
}
