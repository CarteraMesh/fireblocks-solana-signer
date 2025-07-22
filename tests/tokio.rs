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

// #[tokio::test]
// async fn test_actix_web_integration() {
//     use {
//         actix_web::{App, HttpResponse, Responder, test, web},
//         std::sync::Arc,
//     };
//
//     // Mock handler that uses FireblocksSigner
//     async fn test_handler(signer: web::Data<Arc<FireblocksSigner>>) -> impl
// Responder {         // Test that we can get the public key without issues
//         match signer.try_pubkey() {
//             Ok(pubkey) => HttpResponse::Ok().json(serde_json::json!({
//                 "status": "success",
//                 "pubkey": pubkey.to_string()
//             })),
//             Err(e) =>
// HttpResponse::InternalServerError().json(serde_json::json!({
// "status": "error",                 "error": e.to_string()
//             })),
//         }
//     }
//
//     // Create a mock signer with a keypair (to avoid actual Fireblocks calls
// in     // tests)
//     let keypair = solana_keypair::Keypair::new();
//     let signer = FireblocksSigner::builder()
//         .vault_id("test".to_string())
//         .asset(SOL_TEST)
//         .pk(keypair.pubkey())
//         .poll_config(PollConfig::default())
//         .keypair(Some(std::sync::Arc::new(keypair)))
//         .build();
//
//     let signer_data = Arc::new(signer);
//
//     // Create test app
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(signer_data.clone()))
//             .route("/test", web::get().to(test_handler)),
//     )
//     .await;
//
//     // Test the endpoint
//     let req = test::TestRequest::get().uri("/test").to_request();
//     let resp = test::call_service(&app, req).await;
//
//     assert!(resp.status().is_success());
//
//     let body_bytes = test::read_body(resp).await;
//     let response: serde_json::Value =
// serde_json::from_slice(&body_bytes).unwrap();
//
//     assert_eq!(response["status"], "success");
//     assert!(response["pubkey"].is_string());
// }
//
// #[tokio::test]
// async fn test_thread_spawn_approach() {
//     setup();
//
//     // Test that our std::thread::spawn approach works in tokio test context
//     let keypair = solana_keypair::Keypair::new();
//     let signer = FireblocksSigner::builder()
//         .vault_id("test".to_string())
//         .asset(SOL_TEST)
//         .pk(keypair.pubkey())
//         .poll_config(PollConfig::default())
//         .keypair(Some(Arc::new(keypair)))
//         .build();
//
//     // Test signing a message
//     let test_message = b"test message for signing";
//     let result = signer.try_sign_message(test_message);
//
//     assert!(result.is_ok(), "Signing should succeed: {:?}", result);
//
//     let signature = result.unwrap();
//     assert_ne!(signature.to_string(), "11111111111111111111111111111111");
// }
