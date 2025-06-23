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
    std::{fmt::Debug, time::Duration},
};

#[derive(Clone)]
pub struct Client {
    url: String,
    client: reqwest::blocking::Client,
    jwt: JwtSigner,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[fireblocks-client]")
    }
}

// mod poll;
// mod transfer;

pub struct ClientBuilder {
    api_key: String,
    timeout: Duration,
    connect_timeout: Duration,
    user_agent: String,
    secret: Vec<u8>,
    url: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            timeout: Duration::from_secs(15),
            connect_timeout: Duration::from_secs(5),
            user_agent: format!("fireblocks-sdk-rs {}", env!["CARGO_PKG_VERSION"]),
            secret: vec![],
            url: String::from(FIREBLOCKS_API),
        }
    }
}

impl ClientBuilder {
    pub fn new(api_key: &str, secret: &[u8]) -> Self {
        Self {
            api_key: String::from(api_key),
            secret: Vec::from(secret),
            ..Default::default()
        }
    }

    #[allow(unused_mut, clippy::return_self_not_must_use)]
    pub fn use_sandbox(mut self) -> Self {
        self.with_url(FIREBLOCKS_SANDBOX_API)
    }

    #[allow(unused_mut, clippy::return_self_not_must_use)]
    pub fn with_sandbox(mut self) -> Self {
        self.with_url(FIREBLOCKS_SANDBOX_API)
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = String::from(url);
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub const fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = String::from(ua);
        self
    }

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
    fn new_with_url(url: &str, client: reqwest::blocking::Client, jwt: JwtSigner) -> Self {
        Self {
            url: String::from(url),
            client,
            jwt,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{path}", self.url)
    }

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

        Ok(serde_json::from_str(&body)?)
    }

    #[tracing::instrument(level = "debug")]
    pub fn address(&self, vault: &str, asset: &str) -> Result<Pubkey> {
        let path = format!("/v1/vault/accounts/{vault}/{asset}/addresses_paginated");
        let url = self.build_url(&path);
        let signed = self.jwt.sign(&path, &[])?;
        let result: VaultAddressesResponse = self.send(self.client.get(url), signed)?;
        if result.addresses.is_empty() {
            return Err(crate::Error::FireblocksNoPubkey(vault.to_string()));
        }
        Ok(result.addresses[0].address)
    }

    #[tracing::instrument(level = "debug", skip(base64_tx))]
    pub fn program_call(
        &self,
        asset_id: &str,
        vault_id: &str,
        base64_tx: String,
    ) -> Result<CreateTransactionResponse> {
        let path = String::from("/v1/transactions");
        let url = self.build_url(&path);
        let extra = ExtraParameters::new(base64_tx);
        let source = SourceTransferPeerPath::new(vault_id.to_string());
        let tx = TransactionRequest::new(asset_id.to_string(), source, extra);
        let body = serde_json::to_vec(&tx)?;
        let signed = self.jwt.sign(&path, &body)?;
        let req = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body);

        self.send(req, signed)
    }

    pub fn get_tx(&self, txid: &str) -> Result<(TransactionResponse, Option<Signature>)> {
        let path = format!("/v1/transactions/{txid}");
        let url = self.build_url(&path);
        let signed = self.jwt.sign(&path, &[])?;
        let result: TransactionResponse = self.send(self.client.get(&url), signed)?;
        let sig: Option<Signature> = Signature::try_from(result.clone()).ok();
        Ok((result, sig))
    }

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
        // maybe last call will be lucky
        self.get_tx(txid)
    }
}
