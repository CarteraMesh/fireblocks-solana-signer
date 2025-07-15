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
mod error;
mod extensions;
mod signer;

pub use {
    asset::*,
    error::Error,
    extensions::*,
    fireblocks_signer_transport::{
        Client,
        ClientBuilder,
        FIREBLOCKS_API,
        FIREBLOCKS_SANDBOX_API,
        TransactionResponse,
        TransactionStatus,
    },
    signer::*,
    solana_pubkey::{Pubkey, pubkey},
    solana_signature::Signature,
    solana_signer::Signer,
    std::str::FromStr,
};

/// Environment variables used by the FireblocksSigner.
#[derive(Debug, Clone, Copy)]
pub enum EnvVar {
    Vault,
    Secret,
    ApiKey,
    Endpoint,
    Pubkey,
    Testnet,
    Devnet,
    PollTimeout,
    PollInterval,
}

impl std::fmt::Display for EnvVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EnvVar::Vault => "FIREBLOCKS_VAULT",
            EnvVar::Secret => "FIREBLOCKS_SECRET",
            EnvVar::ApiKey => "FIREBLOCKS_API_KEY",
            EnvVar::Endpoint => "FIREBLOCKS_ENDPOINT",
            EnvVar::Pubkey => "FIREBLOCKS_PUBKEY",
            EnvVar::Testnet => "FIREBLOCKS_TESTNET",
            EnvVar::Devnet => "FIREBLOCKS_DEVNET",
            EnvVar::PollTimeout => "FIREBLOCKS_POLL_TIMEOUT",
            EnvVar::PollInterval => "FIREBLOCKS_POLL_INTERVAL",
        };
        write!(f, "{name}")
    }
}

impl AsRef<std::ffi::OsStr> for EnvVar {
    fn as_ref(&self) -> &std::ffi::OsStr {
        match self {
            EnvVar::Vault => std::ffi::OsStr::new("FIREBLOCKS_VAULT"),
            EnvVar::Secret => std::ffi::OsStr::new("FIREBLOCKS_SECRET"),
            EnvVar::ApiKey => std::ffi::OsStr::new("FIREBLOCKS_API_KEY"),
            EnvVar::Endpoint => std::ffi::OsStr::new("FIREBLOCKS_ENDPOINT"),
            EnvVar::Pubkey => std::ffi::OsStr::new("FIREBLOCKS_PUBKEY"),
            EnvVar::Testnet => std::ffi::OsStr::new("FIREBLOCKS_TESTNET"),
            EnvVar::Devnet => std::ffi::OsStr::new("FIREBLOCKS_DEVNET"),
            EnvVar::PollTimeout => std::ffi::OsStr::new("FIREBLOCKS_POLL_TIMEOUT"),
            EnvVar::PollInterval => std::ffi::OsStr::new("FIREBLOCKS_POLL_INTERVAL"),
        }
    }
}

impl From<(EnvVar, std::env::VarError)> for Error {
    fn from((env_var, _): (EnvVar, std::env::VarError)) -> Self {
        Error::EnvMissing(env_var.to_string())
    }
}
/// A type alias for [`std::result::Result`] with this crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;
pub const DEFAULT_CLIENT_TIMEOUT: u8 = 15;

/// See [`build_client_and_address_blocking_safe`]
pub fn build_client_safe(builder: ClientBuilder) -> Result<Client> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        if tx.send(builder.build()).is_err() {
            tracing::error!("Failed to send result back to main thread");
        }
    });
    Ok(rx.recv()??)
}

/// Builds a Fireblocks client and retrieves the associated Solana address in a
/// tokio-safe manner.
///
/// This function is specifically designed for applications running in a tokio
/// runtime environment. The underlying `fireblocks_signer_transport::Client`
/// uses blocking HTTP operations via `reqwest` that can cause panics when
/// called directly from within a tokio async context. This function
/// prevents such panics by executing the blocking operations in a separate OS
/// thread.
///
/// # Tokio Runtime Safety
///
/// **Important**: This function is primarily intended for programs running
/// under tokio runtime. The `reqwest` crate's blocking client will panic if
/// used directly in an async tokio context because it attempts to create a new
/// tokio runtime while one is already running. This function solves that
/// problem by:
///
/// 1. Spawning a separate OS thread (not a tokio task)
/// 2. Performing all blocking operations in that thread
/// 3. Using a channel to safely communicate results back to the main thread
/// 4. Including timeout handling to prevent indefinite blocking
///
/// # Parameters
///
/// * `builder` - A configured `ClientBuilder` for creating the Fireblocks
///   client
/// * `vault` - The Fireblocks vault ID to use
/// * `asset` - The asset type (typically Solana) for address derivation
/// * `address` - Optional pre-existing address string. If `None`, the address
///   will be fetched from Fireblocks
///
/// # Returns
///
/// Returns a tuple containing:
/// * `fireblocks_signer_transport::Client` - The configured Fireblocks client
/// * `Pubkey` - The Solana public key/address associated with the vault and
///   asset
///
/// # Errors
///
/// This function can return various errors:
/// * `Error::Timeout` - If client initialization takes longer than the
///   configured timeout
/// * `Error::ThreadPanic` - If the worker thread panics during initialization
/// * `Error::ChannelClosed` - If the communication channel closes unexpectedly
/// * Other `Error` variants from the underlying Fireblocks client or address
///   parsing
///
/// # Example
///
/// ```rust,no_run
/// use fireblocks_solana_signer::{Asset, ClientBuilder, build_client_and_address_blocking_safe};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let builder = ClientBuilder::new("api_key", b"private_key");
///     let vault_id = "vault_123".to_string();
///     let asset = Asset::Sol;
///
///     // Safe to call from within tokio runtime
///     let (client, pubkey) =
///         build_client_and_address_blocking_safe(builder, vault_id, asset, None)?;
///
///     println!("Initialized client with address: {}", pubkey);
///     Ok(())
/// }
/// ```
pub fn build_client_and_address_blocking_safe(
    builder: ClientBuilder,
    vault: String,
    asset: Asset,
    address: Option<String>,
) -> Result<(fireblocks_signer_transport::Client, Pubkey)> {
    let client = build_client_safe(builder)?;
    match address {
        Some(pk) => Ok((client, Pubkey::from_str(&pk)?)),
        None => {
            let (tx, rx) = std::sync::mpsc::channel();
            let handle = std::thread::spawn(move || {
                let result = match client.address(&vault, &asset) {
                    Err(e) => Err(crate::Error::from(e)),
                    Ok(pk) => match Pubkey::from_str(&pk) {
                        Err(e) => Err(crate::Error::from(e)),
                        Ok(pk) => Ok((client, pk)),
                    },
                };
                // Don't ignore send errors
                if tx.send(result).is_err() {
                    tracing::error!("Failed to send result back to main thread");
                }
            });
            tracing::debug!("waiting for client builder response...");

            // Add timeout to prevent infinite blocking
            match rx.recv_timeout(std::time::Duration::from_secs(
                (DEFAULT_CLIENT_TIMEOUT + 5).into(),
            )) {
                Ok(result) => Ok(result?),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    tracing::error!("Client initialization timed out");
                    Err(Error::Timeout(
                        "Client initialization timed out".to_string(),
                    ))
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    // Check if thread panicked
                    if let Err(panic_err) = handle.join() {
                        tracing::error!("Client initialization thread panicked: {:?}", panic_err);
                        Err(Error::ThreadPanic(
                            "Client initialization thread panicked".to_string(),
                        ))
                    } else {
                        Err(Error::ChannelClosed(
                            "Channel disconnected unexpectedly".to_string(),
                        ))
                    }
                }
            }
        }
    }
}
