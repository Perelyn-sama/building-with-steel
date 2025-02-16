use account_data_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "account_data_program",
        account_data_api::ID,
        processor!(account_data_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (banks, payer, blockhash) = setup().await;

    let address_info_keypair = Keypair::new();

    let address_info_data = AddressInfo {
        name: string_to_bytes("Perelyn").unwrap(),
        house_number: 1,
        street: string_to_bytes("Turbine").unwrap(),
        city: string_to_bytes("Solana").unwrap(),
    };

    // Submit initialize transaction.
    let ix = create(
        payer.pubkey(),
        address_info_keypair.pubkey(),
        address_info_data,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &address_info_keypair],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let address = address_info_keypair.pubkey();
    let account = banks.get_account(address).await.unwrap().unwrap();
    // let data = Create::try_from_bytes(&account.data).unwrap();
    let data = Create::try_from_bytes(&account.data[8..]).unwrap();

    println!("account data {:?}", &account.data);
    println!(
        "name: {:?}",
        bytes_to_string::<64>(&data.data.name).unwrap()
    );
    println!(
        "street: {:?}",
        bytes_to_string::<64>(&data.data.street).unwrap()
    );
    println!(
        "city: {:?}",
        bytes_to_string::<64>(&data.data.city).unwrap()
    );

    assert_eq!(account.owner, account_data_api::ID);
    // assert!(false);
    assert_eq!(data.data.name, string_to_bytes::<64>("Perelyn").unwrap());
    assert_eq!(data.data.street, string_to_bytes::<64>("Turbine").unwrap());
    assert_eq!(data.data.city, string_to_bytes::<64>("Solana").unwrap());
}
