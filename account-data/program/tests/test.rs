use account_data_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

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
    let (mut banks, payer, blockhash) = setup().await;

    let address_info_keypair = Keypair::new();

    let address_info_data = AddressInfo {
        name: string_to_bytes("Perelyn").unwrap(),
        house_number: 1,
        street: string_to_bytes("Turbin").unwrap(),
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
    let data = Create::try_from_bytes(&account.data).unwrap();

    println!("account data {:?}", &account.data);
    println!("name {:?}", data.data.name);
    println!("steet {:?}", data.data.street);
    dbg!("data {:?}", data.data);

    assert_eq!(account.owner, account_data_api::ID);
}
