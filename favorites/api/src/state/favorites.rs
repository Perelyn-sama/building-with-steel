use steel::*;

use super::FavoritesAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Favorites {
    pub number: u8,

    pub color: [u8; 16],

    pub hobbies: [[u8; 16]; 3],
}

account!(FavoritesAccount, Favorites);
