use favorites_api::prelude::*;
use steel::*;

pub fn process_set_favorites(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, favorites_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    favorites_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[FAVORITES], &favorites_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // get args
    let args = SetFavorites::try_from_bytes(data)?;

    // Initialize favorites.
    create_program_account::<Favorites>(
        favorites_info,
        system_program,
        signer_info,
        &favorites_api::ID,
        &[FAVORITES],
    )?;
    let favorites = favorites_info.as_account_mut::<SetFavorites>(&favorites_api::ID)?;
    favorites.data = args.data;

    Ok(())
}
