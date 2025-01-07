use crate::instructions::user_account::create_user_account;
use solana_program::{
    account_info::{AccountInfo},
    pubkey::Pubkey,
    entrypoint::ProgramResult,
};

pub fn handle_login(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    create_user_account(program_id, accounts)
}
