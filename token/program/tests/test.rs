use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, transaction::Transaction,
};
use steel::*;
use token_api::prelude::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "token_program",
        token_api::ID,
        processor!(token_program::process_instruction),
    );

    program_test.add_program("token_metadata", mpl_token_metadata::ID, None);

    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    let mint_keypair = Keypair::new();

    let name = string_to_bytes::<32>("ANATOLY").unwrap();
    let symbol = string_to_bytes::<8>("MERT").unwrap();
    let uri = string_to_bytes::<128>("blah blah blah").unwrap();
    let decimals = 9;

    // Submit create transaction.
    let ix = create(
        payer.pubkey(),
        mint_keypair.pubkey(),
        name,
        symbol,
        uri,
        decimals,
    );
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_keypair],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let serialized_mint_data = banks
        .get_account(mint_keypair.pubkey())
        .await
        .unwrap()
        .unwrap()
        .data;

    let mint_data = spl_token::state::Mint::unpack(&serialized_mint_data).unwrap();
    assert!(mint_data.is_initialized);
    assert_eq!(mint_data.mint_authority.unwrap(), payer.pubkey());
    assert_eq!(mint_data.decimals, decimals);

    // Submit initialize transaction.
    let ix = mint(payer.pubkey(), payer.pubkey(), mint_keypair.pubkey(), 100);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    let ata = spl_associated_token_account::get_associated_token_address(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
    );

    let serialized_ata_info = banks.get_account(ata).await.unwrap().unwrap().data;
    let ata_info = spl_token::state::Account::unpack(&serialized_ata_info).unwrap();
    assert_eq!(ata_info.amount, 100);

    dbg!(ata_info);
    assert!(false);
}
