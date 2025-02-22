mod favorites;

pub use favorites::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum FavoritesAccount {
    Favorites = 0,
}

/// Fetch PDA of the favourites account.
pub fn favorites_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[FAVORITES], &crate::id())
}
