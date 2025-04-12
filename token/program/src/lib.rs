mod create;
mod mint;

pub use create::*;
pub use mint::*;

use steel::*;
use token_api::prelude::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&token_api::ID, program_id, data)?;

    match ix {
        TokenInstruction::Create => process_create(accounts, data)?,
        TokenInstruction::Mint => process_mint(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
