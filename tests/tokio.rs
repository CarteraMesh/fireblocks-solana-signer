#![cfg(not(feature = "agave"))]

mod utils;
use {
    fireblocks_solana_signer::*,
    solana_message::{Message, VersionedMessage},
    solana_rpc_client::rpc_client::SerializableTransaction,
    solana_transaction::{Transaction, versioned::VersionedTransaction},
    utils::{memo, setup, signer},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_tokio() -> anyhow::Result<()> {
    setup();
    let (signer, _) = signer()?;
    let rpc = solana_rpc_client::nonblocking::rpc_client::RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let hash = rpc.get_latest_blockhash().await?;
    let message = Message::new(&[memo("fireblocks signer tokio")], Some(&signer.pk));
    assert!(signer.is_interactive());

    // Sign the transaction directly - no need for spawn_blocking as try_sign
    // will use the tokio version of sign_message
    let mut tx = Transaction::new_unsigned(message);
    tx.try_sign(&[&signer], hash)?;

    let signature = tx.get_signature();
    tracing::info!("Transaction signature: {:?}", signature);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_tokio_single() -> anyhow::Result<()> {
    setup();
    let (signer, _) = signer()?;
    let rpc = solana_rpc_client::nonblocking::rpc_client::RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    );
    let hash = rpc.get_latest_blockhash().await?;
    let message =
        Message::new_with_blockhash(&[memo("fireblocks signer tokio")], Some(&signer.pk), &hash);
    let message = VersionedMessage::Legacy(message);
    let tx = VersionedTransaction::try_new(message, &[&signer])?;
    let signature = tx.get_signature();
    tracing::info!("Transaction signature: {:?}", signature);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_builder() -> anyhow::Result<()> {
    setup();
    let _ = FireblocksSigner::new();
    Ok(())
}
