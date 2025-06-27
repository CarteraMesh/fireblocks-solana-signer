#![doc = include_str!("../README.md")]
//! ⚠️ IMPORTANT: Automatic Transaction Broadcasting
//!
//! **This signer automatically broadcasts transactions to the Solana network.**
//! When you call any signing method (like `try_sign`), Fireblocks will:
//!
//! 1. Sign the transaction with your private key
//! 2. **Automatically broadcast the signed transaction to the network**
//! 3. Return the signature to your application
//!
//! This is a **purposeful security design decision** by Fireblocks to ensure
//! transaction integrity. **You do not need to (and should not) broadcast the
//! transaction yourself** after signing.
//!
//! The transaction is already on-chain when the signing method returns
//! successfully!

mod asset;
mod client;
mod error;
mod extensions;
mod jwt;
mod models;
mod signer;
mod util;

pub use {asset::*, client::*, error::Error, extensions::*, models::*, signer::*};

/// A type alias for [`std::result::Result`] with this crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// The production Fireblocks API endpoint.
pub const FIREBLOCKS_API: &str = "https://api.fireblocks.io";

/// The sandbox Fireblocks API endpoint for testing.
pub const FIREBLOCKS_SANDBOX_API: &str = "https://sandbox-api.fireblocks.io";

#[cfg(test)]
mod test {

    use {
        super::*,
        base64::prelude::*,
        solana_message::Message,
        solana_pubkey::{Pubkey, pubkey},
        solana_rpc_client::rpc_client::{RpcClient, SerializableTransaction},
        solana_sdk::instruction::Instruction,
        solana_signer::Signer,
        solana_transaction::{Transaction, versioned::VersionedTransaction},
        std::{
            env,
            sync::{Arc, Once},
            time::Duration,
        },
        tracing_subscriber::{EnvFilter, fmt::format::FmtSpan},
    };
    static INIT: Once = Once::new();
    const LOOKUP: Pubkey = pubkey!("24DJ3Um2ekF2isQVMZcNHusmzLMMUS1oHQXhpPkVX7WV");
    #[allow(dead_code)]
    const USDC: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    const TO: Pubkey = pubkey!("E4SfgGV2v9GLYsEkCQhrrnFbBcYmAiUZZbJ7swKGzZHJ");

    #[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
    pub fn setup() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_target(true)
                .with_level(true)
                .with_span_events(FmtSpan::CLOSE)
                .with_env_filter(EnvFilter::from_default_env())
                .init();

            if env::var("CI").is_err() {
                // only load .env if not in CI
                let env = dotenvy::dotenv();
                if env.is_err() {
                    tracing::debug!("no .env file");
                }
            }
        });
    }

    fn memo(message: &str) -> Instruction {
        Instruction {
            program_id: spl_memo::id(),
            accounts: vec![],
            data: message.as_bytes().to_vec(),
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
                .with_user_agent("fireblocks-solana-signer-test")
                .with_timeout(Duration::from_secs(15))
                .build()?,
            rpc,
        ))
    }

    fn signer() -> anyhow::Result<(FireblocksSigner, Arc<RpcClient>)> {
        let (client, rpc) = clients()?;
        let poll = PollConfig::builder()
            .timeout(Duration::from_secs(15))
            .interval(Duration::from_secs(3))
            .callback(|t| tracing::info!("{}", t))
            .build();
        let pk = client.address("0", "SOL_TEST")?;
        tracing::info!("using pubkey {}", pk);

        let signer = FireblocksSigner::builder()
            .client(client)
            .pk(pk)
            .vault_id("0".to_string())
            .asset(SOL_TEST)
            .poll_config(poll)
            .build();

        Ok((signer, rpc))
    }

    #[test]
    fn test_client() -> anyhow::Result<()> {
        setup();
        let (client, rpc) = clients()?;
        let pk = client.address("0", "SOL_TEST")?;
        tracing::info!("using pubkey {}", pk);
        let hash = rpc.get_latest_blockhash()?;
        let message = Message::new_with_blockhash(&[memo("fireblocks signer")], Some(&pk), &hash);
        let tx = Transaction::new_unsigned(message);
        let base64_tx = BASE64_STANDARD.encode(bincode::serialize(&tx)?);
        let resp = client.program_call("SOL_TEST", "0", base64_tx)?;
        tracing::info!("txid {resp}");
        let (resp, sig) = client.poll(
            &resp.id,
            std::time::Duration::from_secs(90),
            Duration::from_secs(7),
            |t| tracing::info!("transaction status {t}"),
        )?;
        assert!(sig.is_some());
        let sig = sig.unwrap_or_default();
        tracing::info!("sig {sig} txid {}", resp.id);
        Ok(())
    }

    #[test]
    fn test_signer_legacy() -> anyhow::Result<()> {
        setup();
        let (signer, rpc) = signer()?;
        let hash = rpc.get_latest_blockhash()?;
        let message = Message::new(&[memo("fireblocks signer")], Some(&signer.pk));
        let mut tx = Transaction::new_unsigned(message);
        assert!(signer.is_interactive());
        tx.try_sign(&[&signer], hash)?;
        tracing::info!("sig {}", tx.get_signature());
        Ok(())
    }

    #[test]
    fn test_signer_versioned() -> anyhow::Result<()> {
        setup();
        let (signer, rpc) = signer()?;
        let instructions = vec![
            memo("fireblocks signer versioned"),
            memo("lookup this"),
            solana_sdk::system_instruction::transfer(&signer.pk, &TO, 1),
        ];
        let hash = rpc.get_latest_blockhash()?;
        let alt = crate::util::lookup(&rpc, &[LOOKUP])?;
        let mut tx = VersionedTransaction::new_unsigned_v0(&signer.pk, &instructions, &alt, hash)?;
        tx.try_sign(&[&signer], None)?;
        tracing::info!("sig {}", tx.get_signature());
        Ok(())
    }

    #[test]
    fn test_env() -> anyhow::Result<()> {
        setup();
        let _ = FireblocksSigner::from_env(None)?;
        let _ = FireblocksSigner::from_env(Some(|t| println!("{t}")))?;
        Ok(())
    }
}
