#![allow(unexpected_cfgs)]
mod create;

use create::*;

use account_data_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&account_data_api::ID, program_id, data)?;

    match ix {
        AccountDataInstruction::Create => process_create(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
