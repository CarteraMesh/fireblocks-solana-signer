use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

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
