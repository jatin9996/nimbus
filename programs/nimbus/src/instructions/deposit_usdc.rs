use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke_signed,
};
use spl_token::instruction::transfer;
use anchor_lang::solana_program::program_error::ProgramError;

pub fn deposit_usdc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let source_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;
    let authority_account = next_account_info(accounts_iter)?;

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
