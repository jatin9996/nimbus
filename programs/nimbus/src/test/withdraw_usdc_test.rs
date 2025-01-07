use super::*;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program_test::*;
use solana_sdk::account::Account;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_usdc() {
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
        accounts.push(account_info.clone()); // source_account
        accounts.push(account_info.clone()); // destination_account
        let mut authority_account = account_info.clone();
        authority_account.is_signer = true;
        accounts.push(authority_account); // authority_account

        let amount = 5000_u64;

        assert_eq!(
            withdraw_usdc(&program_id, &accounts, amount),
            Ok(())
        );
    }

    #[test]
    fn test_withdraw_usdc_zero_amount() {
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
        accounts.push(account_info.clone()); // source_account
        accounts.push(account_info.clone()); // destination_account
        let mut authority_account = account_info;
        authority_account.is_signer = true;
        accounts.push(authority_account); // authority_account

        let amount = 0_u64;

        assert_eq!(
            withdraw_usdc(&program_id, &accounts, amount),
            Err(ProgramError::InvalidArgument)
        );
    }

    #[test]
    fn test_withdraw_usdc_large_amount_unauthorized() {
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
        accounts.push(account_info.clone()); // source_account
        accounts.push(account_info.clone()); // destination_account
        let mut authority_account = account_info.clone();
        authority_account.is_signer = false; // Not a signer
        accounts.push(authority_account); // authority_account

        let amount = 10_000_001_u64; // Just above the threshold

        assert_eq!(
            withdraw_usdc(&program_id, &accounts, amount),
            Err(ProgramError::MissingRequiredSignature)
        );
    }
}
