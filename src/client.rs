//! Fireblocks API client implementation.
//!
//! This module provides the core client functionality for interacting with the
//! Fireblocks API. The [`Client`] struct handles authentication, request
//! signing, and communication with Fireblocks services for transaction
//! creation, signing, and status polling.
//!
//! The client supports both production and sandbox environments, with
//! configurable timeouts, user agents, and connection parameters through the
//! [`ClientBuilder`].

use {
    crate::{
        CreateTransactionResponse,
        ExtraParameters,
        FIREBLOCKS_API,
        FIREBLOCKS_SANDBOX_API,
        Result,
        SourceTransferPeerPath,
        TransactionRequest,
        TransactionResponse,
        TransactionStatus,
        jwt::JwtSigner,
        models::VaultAddressesResponse,
    },
    jsonwebtoken::EncodingKey,
    reqwest::blocking::RequestBuilder,
    serde::de::DeserializeOwned,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    std::{
        fmt::{Debug, Display},
        time::Duration,
    },
};

/// A client for interacting with the Fireblocks API.
///
/// The [`Client`] handles all communication with Fireblocks services,
/// including:
/// - JWT-based authentication and request signing
/// - Transaction creation and submission
/// - Address retrieval from vaults
/// - Transaction status polling
/// - Error handling and response parsing
///
/// Clients are created using the [`ClientBuilder`] which allows configuration
/// of timeouts, endpoints, and authentication credentials.
#[derive(Clone, Default)]
pub struct Client {
    /// The base URL for the Fireblocks API endpoint.
    url: String,
    /// The underlying HTTP client for making requests.
    client: reqwest::blocking::Client,
    /// JWT signer for authenticating requests.
    jwt: JwtSigner,
}

impl Debug for Client {
    /// Formats the client for debugging without exposing sensitive information.
    ///
    /// This implementation avoids logging API keys, secrets, or other sensitive
    /// authentication data that might be present in the client.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[fireblocks-client]")
    }
}

// mod poll;
// mod transfer;

/// Builder for configuring and creating Fireblocks API clients.
///
/// The [`ClientBuilder`] provides a fluent interface for configuring various
/// aspects of the Fireblocks client, including authentication credentials,
/// network timeouts, API endpoints, and user agent strings.
///
/// Use [`ClientBuilder::new`] to create a builder with the required API key
/// and secret, then chain configuration methods before calling [`build`] to
/// create the final [`Client`].
///
/// [`build`]: ClientBuilder::build
pub struct ClientBuilder {
    /// The Fireblocks API key (UUID format).
    api_key: String,
    /// Request timeout duration.
    timeout: Duration,
    /// Connection timeout duration.
    connect_timeout: Duration,
    /// User agent string for HTTP requests.
    user_agent: String,
    /// RSA private key for JWT signing (PEM format).
    secret: Vec<u8>,
    /// Base URL for the Fireblocks API.
    url: String,
}

impl Default for ClientBuilder {
    /// Creates a default client builder configuration.
    ///
    /// Default values:
    /// - `timeout`: 15 seconds
    /// - `connect_timeout`: 5 seconds
    /// - `user_agent`: "fireblocks-sdk-rs {version}"
    /// - `url`: Production Fireblocks API endpoint
    /// - `api_key` and `secret`: Empty (must be set via [`new`])
    ///
    /// [`new`]: ClientBuilder::new
    fn default() -> Self {
        Self {
            api_key: String::new(),
            timeout: Duration::from_secs(15),
            connect_timeout: Duration::from_secs(5),
            user_agent: format!("{} {}", env!["CARGO_PKG_NAME"], env!["CARGO_PKG_VERSION"]),
            secret: vec![],
            url: String::from(FIREBLOCKS_API),
        }
    }
}

impl ClientBuilder {
    /// Creates a new client builder with the required authentication
    /// credentials.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Fireblocks API key (UUID format)
    /// * `secret` - The RSA private key in PEM format as bytes
    ///
    /// # Returns
    ///
    /// Returns a new [`ClientBuilder`] with the provided credentials and
    /// default settings.
    pub fn new(api_key: &str, secret: &[u8]) -> Self {
        Self {
            api_key: String::from(api_key),
            secret: Vec::from(secret),
            ..Default::default()
        }
    }

    /// Configures the client to use the Fireblocks sandbox environment.
    ///
    /// This is an alias for [`with_sandbox`] provided for compatibility.
    ///
    /// [`with_sandbox`]: ClientBuilder::with_sandbox
    #[allow(unused_mut, clippy::return_self_not_must_use)]
    pub fn use_sandbox(mut self) -> Self {
        self.with_url(FIREBLOCKS_SANDBOX_API)
    }

    /// Configures the client to use the Fireblocks sandbox environment.
    ///
    /// This sets the API endpoint to the sandbox URL for testing purposes.
    /// Sandbox transactions do not affect real assets or balances.
    #[allow(unused_mut, clippy::return_self_not_must_use)]
    pub fn with_sandbox(mut self) -> Self {
        self.with_url(FIREBLOCKS_SANDBOX_API)
    }

    /// Sets a custom API endpoint URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL for the Fireblocks API endpoint
    ///
    /// # Returns
    ///
    /// Returns the builder for method chaining.
    #[allow(clippy::return_self_not_must_use)]
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = String::from(url);
        self
    }

    /// Sets the request timeout duration.
    ///
    /// This controls how long the client will wait for a response from
    /// the Fireblocks API before timing out.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The maximum duration to wait for API responses
    ///
    /// # Returns
    ///
    /// Returns the builder for method chaining.
    #[allow(clippy::return_self_not_must_use)]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the connection timeout duration.
    ///
    /// This controls how long the client will wait when establishing
    /// a connection to the Fireblocks API.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The maximum duration to wait for connection establishment
    ///
    /// # Returns
    ///
    /// Returns the builder for method chaining.
    #[allow(clippy::return_self_not_must_use)]
    pub const fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Sets a custom user agent string for HTTP requests.
    ///
    /// # Arguments
    ///
    /// * `ua` - The user agent string to use in HTTP headers
    ///
    /// # Returns
    ///
    /// Returns the builder for method chaining.
    #[allow(clippy::return_self_not_must_use)]
    pub fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = String::from(ua);
        self
    }

    /// Builds the configured [`Client`].
    ///
    /// This method creates the JWT signer from the provided RSA key,
    /// configures the HTTP client with the specified timeouts and user agent,
    /// and returns a ready-to-use Fireblocks client.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the configured [`Client`] on success.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The RSA private key is invalid or cannot be parsed
    /// - The HTTP client cannot be configured
    /// - The JWT signer cannot be created
    pub fn build(self) -> Result<Client> {
        let key = EncodingKey::from_rsa_pem(&self.secret[..])?;
        let signer = JwtSigner::new(key, &self.api_key);
        let r = reqwest::blocking::ClientBuilder::new()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .user_agent(String::from(&self.user_agent))
            .build()
            .unwrap_or_default();
        Ok(Client::new_with_url(&self.url, r, signer))
    }
}

impl Client {
    /// Creates a new client with the specified URL, HTTP client, and JWT
    /// signer.
    ///
    /// This is an internal constructor used by the [`ClientBuilder`].
    /// Use [`ClientBuilder`] to create clients instead of calling this
    /// directly.
    fn new_with_url(url: &str, client: reqwest::blocking::Client, jwt: JwtSigner) -> Self {
        Self {
            url: String::from(url),
            client,
            jwt,
        }
    }

    /// Builds a complete API URL from a path.
    ///
    /// # Arguments
    ///
    /// * `path` - The API path to append to the base URL
    ///
    /// # Returns
    ///
    /// Returns the complete URL string.
    fn build_url(&self, path: &str) -> String {
        format!("{}{path}", self.url)
    }

    /// Sends an authenticated HTTP request and deserializes the response.
    ///
    /// This method handles the common pattern of adding authentication headers,
    /// sending the request, checking the response status, and deserializing
    /// the JSON response body.
    ///
    /// # Arguments
    ///
    /// * `req` - The HTTP request builder
    /// * `jwt` - The JWT token for authentication
    ///
    /// # Returns
    ///
    /// Returns the deserialized response on success.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The HTTP request fails
    /// - The server returns an error status
    /// - The response body cannot be deserialized
    fn send<T: DeserializeOwned>(&self, req: RequestBuilder, jwt: String) -> Result<T> {
        let resp = req
            .header("Authorization", jwt)
            .header("X-API-KEY", self.jwt.api_key())
            .send()?;
        let status = resp.status();
        let body = resp.text()?;
        if !status.is_success() {
            return Err(crate::Error::FireblocksServerError(body));
        }

        tracing::trace!("body response: {body}");
        let result: serde_json::Result<T> = serde_json::from_str(&body);
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(crate::Error::JsonParseErr(format!(
                "Error {e}\nFailed to parse\n{body}"
            ))),
        }
    }

    /// Retrieves the public key address for a specific vault and asset.
    ///
    /// This method queries the Fireblocks API to get the first address
    /// associated with the specified vault and asset combination.
    ///
    /// # Arguments
    ///
    /// * `vault` - The vault ID to query
    /// * `asset` - The asset identifier (e.g., "SOL", "SOL_TEST")
    ///
    /// # Returns
    ///
    /// Returns the [`Pubkey`] associated with the vault and asset.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The API request fails
    /// - The vault or asset doesn't exist
    /// - No addresses are found for the vault/asset combination
    /// - The response cannot be parsed
    #[tracing::instrument(level = "debug")]
    pub fn address(&self, vault: &str, asset: impl AsRef<str> + Display + Debug) -> Result<Pubkey> {
        let path = format!("/v1/vault/accounts/{vault}/{asset}/addresses_paginated");
        let url = self.build_url(&path);
        let signed = self.jwt.sign(&path, &[])?;
        let result: VaultAddressesResponse = self.send(self.client.get(url), signed)?;
        if result.addresses.is_empty() {
            return Err(crate::Error::FireblocksNoPubkey(vault.to_string()));
        }
        Ok(result.addresses[0].address)
    }

    /// Submits a Solana transaction to Fireblocks for signing and broadcasting.
    ///
    /// This method creates a Fireblocks transaction request with the provided
    /// base64-encoded Solana transaction. Fireblocks will sign the transaction
    /// and automatically broadcast it to the Solana network.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The asset identifier (e.g., "SOL", "SOL_TEST")
    /// * `vault_id` - The vault ID containing the signing key
    /// * `base64_tx` - The base64-encoded serialized Solana transaction
    ///
    /// # Returns
    ///
    /// Returns a [`CreateTransactionResponse`] containing the transaction ID
    /// and initial status information.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The API request fails
    /// - The transaction format is invalid
    /// - The vault or asset doesn't exist
    /// - Fireblocks rejects the transaction
    #[tracing::instrument(level = "debug", skip(base64_tx))]
    pub fn program_call(
        &self,
        asset_id: impl AsRef<str> + Debug,
        vault_id: &str,
        base64_tx: String,
    ) -> Result<CreateTransactionResponse> {
        let path = String::from("/v1/transactions");
        let url = self.build_url(&path);
        let extra = ExtraParameters::new(base64_tx);
        let source = SourceTransferPeerPath::new(vault_id.to_string());
        let tx = TransactionRequest::new(asset_id.as_ref().to_string(), source, extra);
        let body = serde_json::to_vec(&tx)?;
        let signed = self.jwt.sign(&path, &body)?;
        let req = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body);

        self.send(req, signed)
    }

    /// Retrieves the current status and details of a transaction.
    ///
    /// This method queries Fireblocks for the current state of a transaction,
    /// including its status, signatures, and other metadata.
    ///
    /// # Arguments
    ///
    /// * `txid` - The Fireblocks transaction ID
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - [`TransactionResponse`] with full transaction details
    /// - [`Option<Signature>`] containing the Solana signature if available
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The API request fails
    /// - The transaction ID doesn't exist
    /// - The response cannot be parsed
    pub fn get_tx(&self, txid: &str) -> Result<(TransactionResponse, Option<Signature>)> {
        let path = format!("/v1/transactions/{txid}");
        let url = self.build_url(&path);
        let signed = self.jwt.sign(&path, &[])?;
        let result: TransactionResponse = self.send(self.client.get(&url), signed)?;
        let sig: Option<Signature> = Signature::try_from(result.clone()).ok();
        Ok((result, sig))
    }

    /// Polls a transaction until it reaches a final state or times out.
    ///
    /// This method repeatedly checks the transaction status at the specified
    /// interval until the transaction completes, fails, or the timeout is
    /// reached. The callback function is called on each polling iteration
    /// with the current transaction status.
    ///
    /// # Arguments
    ///
    /// * `txid` - The Fireblocks transaction ID to poll
    /// * `timeout` - Maximum time to wait for transaction completion
    /// * `interval` - Time to wait between polling requests
    /// * `callback` - Function called with each transaction status update
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - [`TransactionResponse`] with the final transaction state
    /// - [`Option<Signature>`] containing the Solana signature if available
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - Any individual status check fails
    /// - The transaction cannot be retrieved
    ///
    /// # Behavior
    ///
    /// The method considers these statuses as final:
    /// - `Blocked`, `Cancelled`, `Cancelling` - Transaction was stopped
    /// - `Completed`, `Confirming` - Transaction succeeded
    /// - `Failed`, `Rejected` - Transaction failed
    ///
    /// All other statuses are considered in-progress and will continue polling.
    pub fn poll(
        &self,
        txid: &str,
        timeout: std::time::Duration,
        interval: std::time::Duration,
        callback: impl Fn(&TransactionResponse),
    ) -> Result<(TransactionResponse, Option<Signature>)> {
        let deadline = std::time::Instant::now() + timeout;

        loop {
            let (result, sig) = self.get_tx(txid)?;
            match &result.status {
                TransactionStatus::Blocked
                | TransactionStatus::Cancelled
                | TransactionStatus::Cancelling
                | TransactionStatus::Completed
                | TransactionStatus::Confirming
                | TransactionStatus::Failed
                | TransactionStatus::Rejected => {
                    return Ok((result, sig));
                }
                _ => {
                    callback(&result);
                    // Check if we have time for another iteration
                    let now = std::time::Instant::now();
                    // Sleep for the interval or remaining time, whichever is shorter
                    let remaining = deadline - now;
                    let sleep_duration = interval.min(remaining);
                    std::thread::sleep(sleep_duration);

                    if now >= deadline {
                        tracing::warn!(
                            "timeout while waiting for transaction confirmation {}",
                            result.id
                        );
                        break;
                    }
                }
            }
        }
        // Maybe last call will be lucky
        self.get_tx(txid)
    }
}
