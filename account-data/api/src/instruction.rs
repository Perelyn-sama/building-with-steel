use steel::*;

use crate::state::AddressInfo;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum AccountDataInstruction {
    Create = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Create {
    pub data: AddressInfo,
}

instruction!(AccountDataInstruction, Create);
