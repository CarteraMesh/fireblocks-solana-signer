use {
    actix_web::{App, HttpResponse, HttpServer, Responder, web},
    fireblocks_solana_signer::FireblocksSigner,
    solana_rpc_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction},
    solana_signer::Signer,
    std::sync::Arc,
};

mod common;

#[derive(Clone)]
struct AppState {
    signer: Arc<FireblocksSigner>,
    rpc: Arc<RpcClient>,
}

async fn test_signer(data: web::Data<AppState>) -> impl Responder {
    tracing::info!("=== Runtime Analysis ===");

    // Check if we're in a Tokio runtime
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        let metrics = handle.metrics();
        tracing::info!("✓ In Tokio runtime");
        tracing::info!("  - num_workers: {}", metrics.num_workers());
        // tracing::info!(
        //     "  - num_blocking_threads: {}",
        //     metrics.num_blocking_threads()
        // );
        // tracing::info!("  - active_tasks_count: {}", metrics.active_tasks_count());
        tracing::info!("  - blocking_queue_depth: {}", metrics.global_queue_depth());

        // Check thread info
        if let Some(thread_name) = std::thread::current().name() {
            tracing::info!("  - thread_name: {}", thread_name);
        } else {
            tracing::info!("  - thread_name: <unnamed>");
        }
        tracing::info!("  - thread_id: {:?}", std::thread::current().id());

        // Check environment variables that might indicate test context
        tracing::info!("=== Environment Check ===");
        tracing::info!("  - CARGO: {:?}", std::env::var("CARGO"));
        tracing::info!(
            "  - RUST_TEST_THREADS: {:?}",
            std::env::var("RUST_TEST_THREADS")
        );

        // Check command line args
        let args: Vec<String> = std::env::args().collect();
        tracing::info!("  - args: {:?}", args);

        // Try to use the signer (this should trigger the error)
        tracing::info!("=== Attempting to use FireblocksSigner ===");
        // Try to sign a dummy message
        match data.rpc.get_latest_blockhash().await {
            Ok(hash) => {
                let message = common::memo(&hash, &data.signer, "fireblocks signer actix");
                let mut tx = solana_sdk::transaction::Transaction::new_unsigned(message);
                match tx.try_sign(&[&data.signer], hash) {
                    Ok(_) => {
                        let signature = *tx.get_signature();
                        tracing::info!("✓ Successfully signed message: {}", signature);
                        HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "signature": signature.to_string(),
                            "runtime_info": {
                                "num_workers": metrics.num_workers(),
                                // "num_blocking_threads": metrics.num_blocking_threads(),
                                "thread_name": std::thread::current().name()
                            }
                        }))
                    }
                    Err(e) => {
                        tracing::error!("✗ Failed to sign message: {}", e);
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "error",
                            "error": e.to_string(),
                            "runtime_info": {
                                "num_workers": metrics.num_workers(),
                                // "num_blocking_threads": metrics.num_blocking_threads(),
                                "thread_name": std::thread::current().name()
                            }
                        }))
                    }
                }
            }
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "error": e.to_string()
            })),
        }
    } else {
        tracing::error!("✗ Not in Tokio runtime");
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "error": "Not in Tokio runtime"
        }))
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing and environment
    common::setup();

    tracing::info!("=== Initializing FireblocksSigner ===");

    // Create signer from environment
    let signer = match FireblocksSigner::try_from_env(None) {
        Ok(signer) => {
            tracing::info!("✓ Successfully created FireblocksSigner");
            tracing::info!("  - Pubkey: {}", signer.try_pubkey().unwrap_or_default());
            Arc::new(signer)
        }
        Err(e) => {
            tracing::error!("✗ Failed to create FireblocksSigner: {}", e);
            return Err(anyhow::format_err!("Failed to create signer: {e}"));
        }
    };
    let rpc = Arc::new(RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    ));

    let app_state = AppState { signer, rpc };

    tracing::info!("=== Starting Actix Web Server ===");
    tracing::info!("Server will be available at: http://127.0.0.1:8080/");
    tracing::info!("Test the endpoint with: curl http://127.0.0.1:8080/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(test_signer))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
