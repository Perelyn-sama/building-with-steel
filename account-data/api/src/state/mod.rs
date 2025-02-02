mod address_info;

pub use address_info::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDataAccount {
    AddressInfo = 0,
}
