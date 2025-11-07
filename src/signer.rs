//! Fireblocks Solana Signer implementation.
//!
//! This module provides the [`FireblocksSigner`] struct, which implements the
//! Solana [`Signer`] trait using Fireblocks as the backend signing service.
//! This allows for secure transaction signing through Fireblocks' custody
//! solution while maintaining compatibility with the Solana ecosystem.
//!
//! # Examples
//!
//! ```no_run
//! use {
//!     fireblocks_solana_signer::FireblocksSigner,
//!     solana_message::Message,
//!     solana_sdk::instruction::Instruction,
//!     solana_transaction::Transaction,
//! };
//!
//! # fn main() -> anyhow::Result<()> {
//! // Create signer from environment variables
//! let signer = FireblocksSigner::try_from_env(None)?;
//!
//! // Create and sign a transaction
//! let instruction = Instruction {
//!     program_id: spl_memo::id(),
//!     accounts: vec![],
//!     data: b"Hello Fireblocks!".to_vec(),
//! };
//! let message = Message::new(&[instruction], Some(&signer.pk));
//! let mut transaction = Transaction::new_unsigned(message);
//! # Ok(())
//! # }
//! ```

mod config;
mod keypair;
mod poll;
use {
    crate::{
        Asset,
        Client,
        ClientBuilder,
        EnvVar,
        Error,
        Result,
        TransactionStatus,
        VersionedTransactionExtension,
    },
    base64::prelude::*,
    solana_keypair::Keypair,
    solana_message::VersionedMessage,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    std::{fmt::Debug, str::FromStr, sync::Arc, time::Duration},
};
pub use {keypair::keypair_from_seed, poll::*};

/// A Solana signer implementation using Fireblocks as the backend signing
/// service.
///
/// This struct implements the Solana [`Signer`] trait, allowing it to be used
/// anywhere a Solana signer is required while leveraging Fireblocks' secure
/// custody solution for private key management and transaction signing.
///
/// The signer handles the complete flow of:
/// 1. Serializing Solana transactions
/// 2. Sending them to Fireblocks for signing
/// 3. Polling for completion
/// 4. Returning the signed transaction
///
/// # Examples
///
/// ```no_run
/// use {fireblocks_solana_signer::FireblocksSigner, solana_signer::Signer};
///
/// # fn main() -> anyhow::Result<()> {
/// // Create from environment variables
/// let signer = FireblocksSigner::try_from_env(None)?;
///
/// // Get the public key
/// let pubkey = signer.try_pubkey()?;
/// println!("Signer public key: {}", pubkey);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Default, bon::Builder)]
pub struct FireblocksSigner {
    /// The Fireblocks vault ID containing the signing key.
    pub vault_id: String,

    /// The asset type (SOL for mainnet, SOL_TEST for devnet/testnet).
    pub asset: Asset,

    /// The public key associated with this signer.
    pub pk: Pubkey,

    /// Configuration for polling transaction status.
    pub poll_config: PollConfig,

    pub keypair: Option<Arc<Keypair>>,

    /// Sign and fireblocks will broadcast the transaction.
    pub broadcast: bool,

    /// The Fireblocks client for API communication.
    client: Option<Client>,
}

impl Debug for FireblocksSigner {
    /// Formats the signer for debugging, showing vault ID and public key.
    ///
    /// This implementation avoids exposing sensitive information like private
    /// keys or API credentials in debug output.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vault: {} [{}]", self.vault_id, self.pk))
    }
}

impl FireblocksSigner {
    pub fn sign_versioned_transaction(&self, tx: &VersionedTransaction) -> Result<Signature> {
        let client = self.client.as_ref().expect(
            "FireblocksSigner must have either a keypair or a Fireblocks client configured",
        );

        let transaction_base64 = BASE64_STANDARD.encode(bincode::serialize(tx)?);

        log::debug!("tx base64 {transaction_base64}");
        let resp = if self.broadcast {
            client.program_call(&self.asset, &self.vault_id, transaction_base64)?
        } else {
            client.sign_only(&self.asset, &self.vault_id, transaction_base64)?
        };
        let (result, sig) = client.poll(
            &resp.id,
            self.poll_config.timeout,
            self.poll_config.interval,
            self.poll_config.callback,
        )?;
        match &result.status {
            // These statuses indicate the transaction is still pending and shouldn't have been
            // returned by polling
            TransactionStatus::Submitted
            | TransactionStatus::Queued
            | TransactionStatus::Pending3RdParty
            | TransactionStatus::PendingSignature
            | TransactionStatus::PendingAuthorization
            | TransactionStatus::Pending3RdPartyManualApproval
            | TransactionStatus::PendingEnrichment
            | TransactionStatus::PendingAmlScreening => {
                return Err(crate::Error::FireblocksNoSig(format!(
                    "txid: {} is still pending with status {} (\"{}\"). This indicates a polling \
                     timeout or configuration issue.",
                    result.id,
                    result.status,
                    result.sub_status.unwrap_or_default(),
                )));
            }

            // These statuses indicate permanent failure
            TransactionStatus::Failed
            | TransactionStatus::Blocked
            | TransactionStatus::Rejected
            | TransactionStatus::Cancelled
            | TransactionStatus::Cancelling => {
                return Err(crate::Error::FireblocksNoSig(format!(
                    "txid: {} failed with status {} substatus: \"{}\" error: {}",
                    result.id,
                    result.status,
                    result.sub_status.unwrap_or_default(),
                    result
                        .error_description
                        .as_ref()
                        .map_or("unknown error", |v| v)
                )));
            }

            // Broadcasting means the transaction is being sent to the network but not yet confirmed
            // This is a transitional state that polling should have waited through
            TransactionStatus::Broadcasting => {
                log::warn!(
                    "txid {} is in Broadcasting state - transaction may not be fully confirmed yet",
                    result.id
                );
                // Continue to check for signature, but this might indicate
                // incomplete confirmation
            }

            // These are the success states where we expect a signature
            TransactionStatus::Completed
            | TransactionStatus::Confirming
            | TransactionStatus::Signed => {
                log::debug!(
                    "Transaction {} completed with status {}",
                    result.id,
                    result.status
                );
            }
        };
        match sig {
            None => Err(crate::Error::FireblocksNoSig(format!(
                "No Signature available for txid {result} {}",
                result
                    .error_description
                    .as_ref()
                    .map_or("unknown error", |v| v)
            ))),
            Some(s) => Ok(Signature::from_str(&s)?),
        }
    }

    /// Signs a transaction message using Fireblocks.
    ///
    /// This method handles the complete signing flow:
    /// 1. Deserializes the message into a versioned transaction
    /// 2. Encodes the transaction as base64
    /// 3. Sends it to Fireblocks for signing
    /// 4. Polls for completion
    /// 5. Returns the signature
    ///
    /// # Arguments
    ///
    /// * `message` - The serialized transaction message to sign
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the [`Signature`] on success, or an
    /// [`Error`] on failure.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The message cannot be deserialized
    /// - The Fireblocks API call fails
    /// - Polling times out
    /// - No signature is returned from Fireblocks
    ///
    /// # Panics
    ///
    /// Panics if neither a keypair nor a Fireblocks client is configured.
    /// This indicates a fundamental configuration error in the signer setup.
    #[tracing::instrument(level = "debug", skip(message))]
    fn sign_transaction(&self, message: &[u8]) -> Result<Signature> {
        let versioned_message: VersionedMessage = bincode::deserialize(message)
            .map_err(|e| Error::InvalidMessage(format!("Failed to deserialize message: {e}")))?;
        let versioned_transaction = VersionedTransaction::new_unsigned(versioned_message);
        self.sign_versioned_transaction(&versioned_transaction)
    }

    /// Creates a new [`FireblocksSigner`] from environment variables.
    ///
    /// This is the primary way to instantiate a signer, reading configuration
    /// from environment variables as documented in the crate README.
    ///
    /// # Required Environment Variables
    ///
    /// - `FIREBLOCKS_VAULT`: Your Fireblocks vault ID
    /// - `FIREBLOCKS_SECRET`: RSA private key of your API user
    /// - `FIREBLOCKS_API_KEY`: UUID of your API user
    /// - `FIREBLOCKS_ENDPOINT`: Fireblocks API endpoint URL
    ///
    /// # Optional Environment Variables
    ///
    /// - `FIREBLOCKS_TESTNET` or `FIREBLOCKS_DEVNET`: Set to use testnet asset
    /// - `FIREBLOCKS_POLL_TIMEOUT`: Polling timeout in seconds (default: 60)
    /// - `FIREBLOCKS_POLL_INTERVAL`: Polling interval in seconds (default: 5)
    ///
    /// # Arguments
    ///
    /// * `f` - Optional callback function for transaction status updates. If
    ///   `None`, uses the default logging callback.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the configured [`FireblocksSigner`] on
    /// success.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - Required environment variables are missing
    /// - The Fireblocks client cannot be created
    /// - The vault address cannot be retrieved
    /// - Environment variable values are invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// // Use default callback
    /// let signer = FireblocksSigner::try_from_env(None)?;
    ///
    /// // Use custom callback
    /// let signer = FireblocksSigner::try_from_env(Some(|response| {
    ///     println!("Custom callback: {:?}", response);
    /// }))?;
    /// # Ok(())
    /// # }
    /// ```
    /// Creates a new `FireblocksSigner` from environment variables.
    ///
    /// # Environment Variables
    ///
    /// - `FIREBLOCKS_VAULT`: The vault ID
    /// - `FIREBLOCKS_TESTNET` or `FIREBLOCKS_DEVNET`: Set to any value if using
    ///   testnet/devnet
    /// - `FIREBLOCKS_SECRET`: The RSA private key
    /// - `FIREBLOCKS_API_KEY`: The API key
    /// - `FIREBLOCKS_ENDPOINT`: The API endpoint
    /// - `FIREBLOCKS_POLL_TIMEOUT`: Timeout in seconds (default: 60)
    /// - `FIREBLOCKS_POLL_INTERVAL`: Polling interval in seconds (default: 5)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// // Create from environment variables
    /// let signer = FireblocksSigner::try_from_env(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from_env(f: Option<fn(&crate::TransactionResponse)>) -> Result<Self> {
        let vault = std::env::var(EnvVar::Vault).map_err(|e| Error::from((EnvVar::Vault, e)))?;
        let asset =
            if std::env::var(EnvVar::Testnet).is_ok() || std::env::var(EnvVar::Devnet).is_ok() {
                crate::SOL_TEST
            } else {
                crate::SOL
            };
        let key = std::env::var(EnvVar::Secret).map_err(|e| Error::from((EnvVar::Secret, e)))?;
        let api = std::env::var(EnvVar::ApiKey).map_err(|e| Error::from((EnvVar::ApiKey, e)))?;
        let address: Option<String> = std::env::var(EnvVar::Pubkey).ok();
        let endpoint =
            std::env::var(EnvVar::Endpoint).map_err(|e| Error::from((EnvVar::Endpoint, e)))?;
        let rsa_pem = key.as_bytes().to_vec();
        let builder = ClientBuilder::new(&api, &rsa_pem)
            .with_url(&endpoint)
            .with_timeout(Duration::from_secs(crate::DEFAULT_CLIENT_TIMEOUT.into()));
        let (client, pk) = crate::build_client_and_address_blocking_safe(
            builder,
            vault.clone(),
            asset.clone(),
            address,
        )?;
        let default_poll = PollConfig::default();
        let poll_timeout = Duration::from_secs(
            std::env::var(EnvVar::PollTimeout)
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
        );
        let poll_interval = Duration::from_secs(
            std::env::var(EnvVar::PollInterval)
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
        );

        let cb = f.unwrap_or(default_poll.callback);
        let poll = PollConfig::builder()
            .timeout(poll_timeout)
            .interval(poll_interval)
            .callback(cb)
            .build();
        Ok(FireblocksSigner::builder()
            .maybe_client(Some(client))
            .vault_id(vault)
            .asset(asset)
            .poll_config(poll)
            .pk(pk)
            .broadcast(false)
            .build())
    }
}

/// Implementation of the Solana [`Signer`] trait for [`FireblocksSigner`].
///
/// This implementation allows the [`FireblocksSigner`] to be used anywhere
/// a Solana signer is required, providing seamless integration with the
/// Solana ecosystem while using Fireblocks for secure key management.
impl Signer for FireblocksSigner {
    /// Returns the public key associated with this signer.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Pubkey)` containing the signer's public key, or a
    /// [`solana_signer::SignerError`] if the public key cannot be retrieved.
    fn try_pubkey(&self) -> std::result::Result<Pubkey, solana_signer::SignerError> {
        Ok(self.pk)
    }

    /// Signs a message using Fireblocks.
    ///
    /// This method implements the core signing functionality, delegating to
    /// the internal [`sign_transaction`] method and converting errors to
    /// the appropriate Solana signer error type.
    ///
    /// # Arguments
    ///
    /// * `message` - The message bytes to sign
    ///
    /// # Returns
    ///
    /// Returns `Ok(Signature)` on successful signing, or a
    /// Signs a message using this signer's keypair.
    ///
    /// Returns `Ok(Signature)` on successful signing, or a
    /// [`solana_signer::SignerError`] on failure.
    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> std::result::Result<Signature, solana_signer::SignerError> {
        match &self.keypair {
            Some(kp) => kp.try_sign_message(message),
            None => {
                let message_vec = message.to_vec();
                let signer = self.clone();

                log::debug!("spawning sign_transaction call with std::thread::spawn");

                // Use std::thread::spawn for universal compatibility across all contexts
                let (tx, rx) = std::sync::mpsc::channel();

                std::thread::spawn(move || {
                    let result = signer.sign_transaction(&message_vec);
                    let final_result =
                        result.map_err(|e| solana_signer::SignerError::Custom(format!("{e}")));
                    let _ = tx.send(final_result);
                });

                log::debug!("waiting for response...");
                // Wait for the result synchronously (could take 2+ minutes)
                rx.recv().unwrap_or_else(|_| {
                    Err(solana_signer::SignerError::Custom(
                        "Channel closed".to_string(),
                    ))
                })
            }
        }
    }

    /// Indicates whether this signer requires user interaction.
    ///
    /// Returns `true` because Fireblocks signing may require approval
    /// workflows or other interactive elements depending on the vault
    /// configuration and transaction policies.
    fn is_interactive(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use {crate::PollConfig, std::time::Duration};

    #[test]
    fn test_poll() {
        let poll = PollConfig::default();
        assert_eq!(poll.timeout, Duration::from_secs(15));
    }
}
