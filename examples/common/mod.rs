use {
    solana_message::Message,
    solana_sdk::{hash::Hash, instruction::Instruction},
    tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
};

#[allow(dead_code)]
pub fn memo(
    hash: &Hash,
    signer: &fireblocks_solana_signer::FireblocksSigner,
    msg: &str,
) -> Message {
    let i = Instruction {
        program_id: spl_memo::id(),
        accounts: vec![],
        data: msg.as_bytes().to_vec(),
    };

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
