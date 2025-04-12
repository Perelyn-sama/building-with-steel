use solana_program::msg;
use solana_program::program_pack::Pack;
use steel::*;
use token_api::prelude::*;

pub fn process_mint(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [mint_authority_info, recipient_info, recipient_ata_info, mint_info, token_program, ata_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // validate
    mint_authority_info.is_signer()?;
    recipient_info.is_writable()?;
    recipient_ata_info.is_empty()?.is_writable()?;
    mint_info.as_mint()?;
    token_program.is_program(&spl_token::ID)?;
    ata_program.is_program(&spl_associated_token_account::ID)?;
    system_program.is_program(&system_program::ID)?;

    // create recipient ata
    create_associated_token_account(
        mint_authority_info,
        recipient_info,
        recipient_ata_info,
        mint_info,
        system_program,
        token_program,
        ata_program,
    )?;

    msg!("create ata");

    let args = Mint::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // mint to
    let instruction = spl_token::instruction::mint_to(
        token_program.key,
        mint_info.key,
        recipient_ata_info.key,
        mint_authority_info.key,
        &[&mint_authority_info.key],
        amount,
    )?;
    let account_infos = &[
        token_program.clone(),
        mint_info.clone(),
        mint_authority_info.clone(),
        recipient_ata_info.clone(),
    ];
    solana_program::program::invoke(&instruction, account_infos)?;
    msg!("mint to ata");

    Ok(())
}
