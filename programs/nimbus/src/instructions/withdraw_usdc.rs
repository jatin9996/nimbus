use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use spl_token::instruction::transfer;
use solana_program::program::invoke_signed;
use serde::{Deserialize};
use std::fs;
use std::str::FromStr;
#[derive(Deserialize)]
struct Config {
    trusted_authorities: Vec<String>,
}

pub fn withdraw_usdc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Load configuration
    let config_data = fs::read_to_string("config.json").expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).expect("JSON was not well-formatted");

    // Convert string public keys to Pubkey
    let trusted_authorities: Vec<Pubkey> = config.trusted_authorities
        .iter()
        .map(|key| Pubkey::from_str(key).unwrap())
        .collect();

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    let accounts_iter = &mut accounts.iter();
    let source_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;
    let authority_account = next_account_info(accounts_iter)?;

    // Ensure the authority account is indeed authorized and is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Additional security checks for large withdrawals
  
    const LARGE_WITHDRAWAL_THRESHOLD: u64 = 10_000_000; 
    if amount > LARGE_WITHDRAWAL_THRESHOLD {
        let mut signature_count = 0;
        for account in accounts {
            if trusted_authorities.contains(account.key) && account.is_signer {
                signature_count += 1;
            }
        }

        const REQUIRED_SIGNATURES: usize = 2;
        if signature_count < REQUIRED_SIGNATURES {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }

    invoke_signed(
        &transfer(
            &spl_token::id(),
            source_account.key,
            destination_account.key,
            authority_account.key,
            &[],
            amount,
        )?,
        &[source_account.clone(), destination_account.clone(), authority_account.clone()],
        &[],
    )?;

    Ok(())
}
