use crate::state::Favorites;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum FavoritesInstruction {
    SetFavorites = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetFavorites {
    pub data: Favorites,
}

instruction!(FavoritesInstruction, SetFavorites);
