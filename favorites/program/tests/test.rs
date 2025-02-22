use favorites_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "favorites_program",
        favorites_api::ID,
        processor!(favorites_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (banks, payer, blockhash) = setup().await;

    let number = 17;
    let color = string_to_bytes::<16>("blue").unwrap();
    let hobbies = [
        string_to_bytes::<16>("reading").unwrap(),
        string_to_bytes::<16>("coding").unwrap(),
        string_to_bytes::<16>("gaming").unwrap(),
    ];

    let data = Favorites {
        number,
        color,
        hobbies,
    };

    // Submit initialize transaction.
    let ix = set_favorites(payer.pubkey(), data);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify favorites was initialized.
    let address = favorites_pda().0;
    let account = banks.get_account(address).await.unwrap().unwrap();
    let favorites = Favorites::try_from_bytes(&account.data).unwrap();

    assert_eq!(account.owner, favorites_api::ID);
    assert_eq!(favorites.number, number);
    assert_eq!(favorites.color, color);
    assert_eq!(favorites.hobbies, hobbies);
}
