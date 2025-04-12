use spl_associated_token_account::get_associated_token_address;
use steel::*;

use crate::prelude::*;

pub fn create(
    user: Pubkey,
    mint: Pubkey,
    name: [u8; 32],
    symbol: [u8; 8],
    uri: [u8; 128],
    decimals: u8,
) -> Instruction {
    let metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Create {
            name,
            symbol,
            uri,
            decimals,
        }
        .to_bytes(),
    }
}

pub fn mint(mint_authoriy: Pubkey, recipient: Pubkey, mint: Pubkey, amount: u64) -> Instruction {
    let recipient_ata = get_associated_token_address(&recipient, &mint);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(mint_authoriy, true),
            AccountMeta::new(recipient, false),
            AccountMeta::new(recipient_ata, false),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Mint {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
