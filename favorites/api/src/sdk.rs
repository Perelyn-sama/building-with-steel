use steel::*;

use crate::prelude::*;

pub fn set_favorites(signer: Pubkey, data: Favorites) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(favorites_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: SetFavorites { data }.to_bytes(),
    }
}
