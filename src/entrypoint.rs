use crate::instruction::{self, MyProgramInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
#[cfg(target_os = "solana")]
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match MyProgramInstruction::try_from(ix_disc)? {
        MyProgramInstruction::Make => {
            log!("Ix:0");
            instruction::process_make(accounts, instruction_data)
        }
        MyProgramInstruction::Take => {
            log!("Ix:1");
            instruction::process_take(accounts)
        }
        MyProgramInstruction::Refund => {
            log!("Ix:2");
            instruction::process_refund(accounts)
        }
    }
}
