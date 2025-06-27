use {
    crate::{Asset, Client, ClientBuilder, Error, Result, VersionedTransactionExtension},
    base64::prelude::*,
    bon::Builder,
    solana_message::VersionedMessage,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    std::{fmt::Debug, time::Duration},
};

#[derive(Clone, Builder)]
pub struct PollConfig {
    pub timeout: Duration,
    pub interval: Duration,
    pub callback: fn(&crate::TransactionResponse),
}

impl Default for PollConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(15),
            interval: Duration::from_secs(5),
            callback: |t| tracing::info!("{}", t),
        }
    }
}

#[derive(Clone, bon::Builder)]
pub struct FireblocksSigner {
    pub vault_id: String,
    pub asset: Asset,
    pub pk: Pubkey,
    pub poll_config: PollConfig,
    client: Client,
}

impl Debug for FireblocksSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vault: {} [{}]", self.vault_id, self.pk))
    }
}

impl FireblocksSigner {
    #[tracing::instrument(level = "debug", skip(message))]
    fn sign_transaction(&self, message: &[u8]) -> Result<Signature> {
        let versioned_message: VersionedMessage = bincode::deserialize(message)
            .map_err(|e| Error::InvalidMessage(format!("Failed to deserialize message: {e}")))?;
        let versioned_transaction = VersionedTransaction::new_unsigned(versioned_message);
        let transaction_base64 =
            BASE64_STANDARD.encode(bincode::serialize(&versioned_transaction)?);

        tracing::debug!("tx base64 {transaction_base64}");
        let resp = self
            .client
            .program_call(&self.asset, &self.vault_id, transaction_base64)?;
        let (result, sig) = self.client.poll(
            &resp.id,
            self.poll_config.timeout,
            self.poll_config.interval,
            self.poll_config.callback,
        )?;
        sig.ok_or_else(|| {
            crate::Error::FireblocksNoSig(format!("No Signature available for txid {result}"))
        })
    }

    pub fn from_env(f: Option<fn(&crate::TransactionResponse)>) -> Result<Self> {
        let vault = std::env::var("FIREBLOCKS_VAULT")?;
        let asset = if std::env::var("FIREBLOCKS_TESTNET").is_ok()
            || std::env::var("FIREBLOCKS_DEVNET").is_ok()
        {
            crate::SOL_TEST
        } else {
            crate::SOL
        };
        let key = std::env::var("FIREBLOCKS_SECRET")?;
        let api = std::env::var("FIREBLOCKS_API_KEY")?;
        let endpoint = std::env::var("FIREBLOCKS_ENDPOINT")?;
        let rsa_pem = key.as_bytes().to_vec();
        let client = ClientBuilder::new(&api, &rsa_pem)
            .with_url(&endpoint)
            .with_user_agent("fireblocks-solana-signer for rust")
            .with_timeout(Duration::from_secs(15))
            .build()?;

        let pk = client.address(&vault, &asset)?;

        let default_poll = PollConfig::default();
        let poll_timeout = Duration::from_secs(
            std::env::var("FIREBLOCKS_POLL_TIMEOUT")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
        );
        let poll_interval = Duration::from_secs(
            std::env::var("FIREBLOCKS_POLL_INTERVAL")
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
            .client(client)
            .vault_id(vault)
            .asset(asset)
            .poll_config(poll)
            .pk(pk)
            .build())
    }
}

impl Signer for FireblocksSigner {
    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> std::result::Result<Signature, solana_signer::SignerError> {
        self.sign_transaction(message)
            .map_err(|e| solana_signer::SignerError::Custom(format!("{e}")))
    }

    fn try_pubkey(&self) -> std::result::Result<Pubkey, solana_signer::SignerError> {
        Ok(self.pk)
    }

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
