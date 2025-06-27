use {
    fireblocks_solana_signer::{Client, ClientBuilder, FireblocksSigner, PollConfig},
    solana_rpc_client::rpc_client::RpcClient,
    std::{sync::Arc, time::Duration},
    tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
};

pub fn setup() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // only load .env if not in CI
    let env = dotenvy::dotenv();
    if env.is_err() {
        tracing::debug!("no .env file");
    }
}

fn clients() -> anyhow::Result<(Client, Arc<RpcClient>)> {
    let api_key: String =
        std::env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY is not set");
    let key: String = std::env::var("FIREBLOCKS_SECRET").expect("FIREBLOCKS_SECRET is not set");
    let rsa_pem = key.as_bytes().to_vec();
    let rpc = Arc::new(RpcClient::new(
        std::env::var("RPC_URL")
            .ok()
            .unwrap_or("https://rpc.ankr.com/solana_devnet".to_string()),
    ));

    Ok((
        ClientBuilder::new(&api_key, &rsa_pem)
            .with_sandbox()
            .with_user_agent("fireblocks-solana-signer")
            .with_timeout(Duration::from_secs(15))
            .build()?,
        rpc,
    ))
}

pub fn signer() -> anyhow::Result<(FireblocksSigner, Arc<RpcClient>)> {
    let (client, rpc) = clients()?;
    let poll = PollConfig::builder()
        .timeout(Duration::from_secs(15))
        .interval(Duration::from_secs(3))
        .callback(|t| println!("{}", t))
        .build();
    let pk = client.address("0", "SOL_TEST")?;
    println!("using pubkey {}", pk);

    let signer = FireblocksSigner::builder()
        .client(client)
        .pk(pk)
        .vault_id("0".to_string())
        .asset("SOL_TEST".to_string())
        .poll_config(poll)
        .build();

    Ok((signer, rpc))
}
