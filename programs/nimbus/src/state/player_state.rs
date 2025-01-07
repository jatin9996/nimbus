use anchor_lang::prelude::*;

#[account]
pub struct PlayerState {
    pub allowed_user: Pubkey,
    pub latest_flip_result: bool, // Stores the result of the latest flip
    pub randomness_account: Pubkey, // Reference to the Switchboard randomness account
    pub current_guess: bool, // The current guess
    pub wager: u64, // The wager amount
    pub bump: u8,
}
