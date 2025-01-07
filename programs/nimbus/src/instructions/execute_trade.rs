use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke_signed,
};
use spl_token::instruction::transfer;
use std::collections::HashSet;

pub fn execute_trade(
    accounts: &[AccountInfo],
    trade_data: &TradeData,
    signatures: &[Pubkey],
) -> ProgramResult {
    // Verify signatures
    let mut signers = HashSet::new();
    for account in accounts {
        if account.is_signer {
            signers.insert(*account.key);
        }
    }

    for signature in signatures {
        if !signers.contains(signature) {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }

    // Execute trade logic, transferring tokens between accounts
    let accounts_iter = &mut accounts.iter();
    let buyer_account = next_account_info(accounts_iter)?;
    let seller_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Transfer tokens from seller to buyer
    invoke_signed(
        &transfer(
            &spl_token::id(),
            seller_account.key,
            buyer_account.key,
            &trade_data.seller,
            &[],
            trade_data.amount,
        )?,
        &[seller_account.clone(), buyer_account.clone(), token_program.clone()],
        &[],
    )?;

    Ok(())
}

pub struct TradeData {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub price: f64,
}
