use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;

// Import the ExecuteContext struct
use crate::instructions::execute_context::ExecuteContext;

declare_id!("GsTBDyqspFMoSciPcuGtsRNsC2UxrgNLpnf6GoxGCokz");

pub type ProgramResult = std::result::Result<(), ProgramError>;

#[program]
pub mod nimbus {
    use super::*;

    pub fn entry_point(ctx: Context<ExecuteContext>, instruction_data: Vec<u8>) -> ProgramResult {
        let instruction = Instructions::try_from_slice(&instruction_data)
            .map_err(|_| ErrorCode::InstructionUnpackingFailed)?;

        match instruction {
            Instructions::Initialize => {
                msg!("Initializing...");
                Ok(())
            },
            // Add other instructions here
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Failed to unpack instruction data.")]
    InstructionUnpackingFailed,
}