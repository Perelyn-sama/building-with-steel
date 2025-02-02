use steel::*;

use super::AccountDataAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AddressInfo {
    pub name: [u8; 64],

    pub house_number: u8,

    pub street: [u8; 64],

    pub city: [u8; 64],
}

impl AddressInfo {
    pub const LEN: usize = std::mem::size_of::<AddressInfo>();
}

account!(AccountDataAccount, AddressInfo);
