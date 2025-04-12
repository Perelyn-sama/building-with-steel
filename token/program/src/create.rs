use solana_program::msg;
use solana_program::program_pack::Pack;
use steel::*;
use token_api::prelude::*;

pub fn process_create(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [user_info, mint_info, metadata_info, token_program, token_metadata_program, system_program, rent_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validate
    user_info.is_signer()?;
    mint_info.is_empty()?.is_signer()?;
    metadata_info.is_empty()?.is_writable()?;
    token_program.is_program(&spl_token::ID)?;
    token_metadata_program.is_program(&mpl_token_metadata::ID)?;
    system_program.is_program(&system_program::ID)?;
    rent_sysvar.is_sysvar(&sysvar::rent::ID)?;

    // create mint account
    create_account(
        user_info,
        mint_info,
        system_program,
        spl_token::state::Mint::LEN,
        &token_program.key,
    )?;

    msg!("create account");

    let args = Create::try_from_bytes(data)?;
    let name = bytes_to_string::<32>(&args.name)?;
    let symbol = bytes_to_string::<8>(&args.symbol)?;
    let uri = bytes_to_string::<128>(&args.uri)?;
    let decimals = args.decimals;

    // initialize mint
    let ix = spl_token::instruction::initialize_mint(
        token_program.key,
        mint_info.key,
        user_info.key,
        Some(user_info.key),
        decimals,
    )?;
    let account_infos = &[
        user_info.clone(),
        mint_info.clone(),
        token_program.clone(),
        rent_sysvar.clone(),
    ];
    solana_program::program::invoke(&ix, account_infos)?;

    msg!("initialize mint");

    // create metadata account
    mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi {
        __program: token_metadata_program,
        metadata: metadata_info,
        mint: mint_info,
        mint_authority: user_info,
        payer: user_info,
        update_authority: (user_info, true),
        system_program,
        rent: Some(rent_sysvar),
        __args: mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
            data: mpl_token_metadata::types::DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        },
    }
    .invoke()?;
    msg!("metadata account created");

    Ok(())
}
