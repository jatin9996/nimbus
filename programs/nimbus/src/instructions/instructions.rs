use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Instructions {
    Initialize,
    Login,
    CreateTokenAccount,
    CreateUserAccount,
    DepositUSDC,  // New instruction for USDC deposit
    WithdrawUSDC,  // New instruction for USDC withdrawal
}
