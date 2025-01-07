use super::*;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program_test::*;
use solana_sdk::account::Account;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_login() {
        let program_id = Pubkey::from_str("Pubkey").unwrap();
        let mut lamports = 0;
        let mut data = vec![0; 100];
        let mut accounts = Vec::new();
        let account_info = AccountInfo::new(
            &Pubkey::new_unique(),
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );
        accounts.push(account_info);

        assert_eq!(
            handle_login(&program_id, &accounts),
            Ok(())
        );
    }
}
