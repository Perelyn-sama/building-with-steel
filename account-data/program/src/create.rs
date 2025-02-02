use account_data_api::prelude::*;
use steel::*;

pub fn process_create(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [payer_info, address_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    payer_info.is_signer()?;

    address_info.is_empty()?.is_writable()?;

    system_program.is_program(&system_program::ID)?;

    create_account_helper(
        payer_info,
        address_info,
        system_program,
        AddressInfo::LEN,
        &account_data_api::ID,
    )?;

    let args = Create::try_from_bytes(data)?;

    let address_info_data = address_info.as_account_mut::<Create>(&account_data_api::ID)?;
    address_info_data.data = args.data;

    Ok(())
}
