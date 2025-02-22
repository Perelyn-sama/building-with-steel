use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum FavoritesError {
    #[error("This is a dummy error")]
    Dummy = 0,
}

error!(FavoritesError);
