use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenInstruction {
    Create = 0,
    Mint = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Create {
    pub name: [u8; 32],
    pub symbol: [u8; 8],
    pub uri: [u8; 128],
    pub decimals: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mint {
    pub amount: [u8; 8],
}

instruction!(TokenInstruction, Create);
instruction!(TokenInstruction, Mint);
