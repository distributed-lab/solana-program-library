#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use solana_program::program_error::PrintProgramError;

use crate::processor;
use crate::error::UpgradeError;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    match processor::process_instruction(program_id, accounts, instruction_data) {
        Ok(()) => Ok(()),
        Err(e) => {
            // catch the error so we can print it
            e.print::<UpgradeError>();
            return Err(e);
        }
    }
}
