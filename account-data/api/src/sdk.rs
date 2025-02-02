use steel::*;

use crate::prelude::*;

pub fn create(payer: Pubkey, address_info_key: Pubkey, data: AddressInfo) -> Instruction {
    // println!("data param: {:?}", data);
    // println!("create as bytes: {:?}", Create { data }.to_bytes());

    // let create_data = Create { data }.to_bytes();
    // println!("{:?}", Create::try_from_bytes(&create_data));

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(address_info_key, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Create { data }.to_bytes(),
    }
}
