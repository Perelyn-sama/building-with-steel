#![allow(unexpected_cfgs)]
mod set_favorites;

use set_favorites::*;

use favorites_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&favorites_api::ID, program_id, data)?;

    match ix {
        FavoritesInstruction::SetFavorites => process_set_favorites(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
