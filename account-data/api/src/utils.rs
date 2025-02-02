use solana_program::rent::Rent;
use steel::*;

pub fn string_to_bytes<const N: usize>(s: &str) -> [u8; N] {
    let mut bytes = [0; N];
    let s_bytes = s.as_bytes();

    // Check length before doing any operations
    // if s_bytes.len() > N {
    //     return Err(ProgramError::Custom(ERROR_STRING_TOO_LONG));
    // }

    let len = s_bytes.len();
    bytes[..len].copy_from_slice(&s_bytes[..len]);
    bytes
}

pub fn bytes_to_string<const N: usize>(bytes: &[u8; N]) -> String {
    // Find the actual length by looking for the first zero or taking full length
    let actual_len = bytes.iter().position(|&b| b == 0).unwrap_or(N);

    // Convert the slice up to actual_len to a string
    String::from_utf8_lossy(&bytes[..actual_len])
        .trim_matches('\0')
        .to_string()
}

/// Creates a new account.
#[inline(always)]
pub fn create_account_helper<'a, 'info>(
    from_pubkey: &'a AccountInfo<'info>,
    to_pubkey: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    space: usize,
    owner: &Pubkey,
) -> ProgramResult {
    let lamports_required = (Rent::get()?).minimum_balance(space);

    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            from_pubkey.key,
            to_pubkey.key,
            lamports_required,
            space as u64,
            owner,
        ),
        &[
            from_pubkey.clone(),
            to_pubkey.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}
