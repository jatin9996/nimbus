use super::*;
use solana_program::account_info::AccountInfo;
use solana_program_test::*;
use solana_sdk::account::Account;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_account() {
        let program_id = Pubkey::from_str("Pubkey").unwrap();
        let mut accounts = Vec::new();
        let account_info = AccountInfo::new(
            &Pubkey::new_unique(),
            false,
            true,
            &mut 0,
            &mut [],
            &program_id,
            false,
            0,
        );
        accounts.push(account_info);

        assert_eq!(
            create_user_account(&program_id, &accounts),
            Ok(())
        );
    }

    #[test]
    fn test_create_token_account() {
        let program_id = Pubkey::from_str("Pubkey").unwrap();
        let mut accounts = Vec::new();
        let account_info = AccountInfo::new(
            &Pubkey::new_unique(),
            false,
            true,
            &mut 0,
            &mut [],
            &program_id,
            false,
            0,
        );
        accounts.push(account_info);

        assert_eq!(
            create_(&program_id, &accounts),
            Ok(())
        );
    }
}
