use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction::initialize_account;

pub fn create_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    let rent = Rent::from_account_info(rent_sysvar)?;
    let lamports = rent.minimum_balance(spl_token::state::Account::LEN);

    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            token_account.key,
            lamports,
            spl_token::state::Account::LEN as u64,
            &spl_token::id(),
        ),
        &[payer_account.clone(), token_account.clone()],
        &[],
    )?;

    invoke_signed(
        &initialize_account(
            &spl_token::id(),
            token_account.key,
            mint_account.key,
            payer_account.key,
        )?,
        &[token_account.clone(), mint_account.clone(), payer_account.clone()],
        &[],
    )?;

    Ok(())
}
