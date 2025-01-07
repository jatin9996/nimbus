use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExecuteContext<'info> {
    #[account(mut)]
    pub program_id: AccountInfo<'info>,

    /// The authority executing the transaction; must be a signer.
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    /// Source account from which funds or other assets will be drawn.
    #[account(mut)]
    pub source_account: AccountInfo<'info>,

    /// Destination account to which funds or other assets will be sent.
    #[account(mut)]
    pub destination_account: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Failed to unpack instruction data.")]
    InstructionUnpackingFailed,
    #[msg("Invalid program ID.")]
    InvalidProgramId,
}
