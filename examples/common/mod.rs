use {
    solana_sdk::{hash::Hash, message::Message, signature::Signer},
    spl_memo_interface::v3::ID,
    tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
};

#[allow(dead_code)]
pub fn memo(
    hash: &Hash,
    signer: &fireblocks_solana_signer::FireblocksSigner,
    msg: &str,
) -> Message {
    let i = spl_memo_interface::instruction::build_memo(&ID, msg.as_bytes(), &[&signer.pubkey()]);
    Message::new_with_blockhash(&[i], Some(&signer.pk), hash)
}

pub fn setup() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let env = dotenvy::dotenv();
    if env.is_err() {
        tracing::debug!("no .env file");
    }
}
