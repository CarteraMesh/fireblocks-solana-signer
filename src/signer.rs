use {
    crate::{Client, Error, Result, VersionedTransactionExtension},
    base64::prelude::*,
    solana_message::VersionedMessage,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    std::{fmt::Debug, time::Duration},
};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FireblocksSigner {
    pub vault_id: String,
    pub asset: String,
    pub pk: Pubkey,
    poll_config: PollConfig,
    client: Client,
}

impl Debug for FireblocksSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vault: {} [{}]", self.vault_id, self.pk))
    }
}

impl FireblocksSigner {
    pub fn init(vault_id: String, asset: &str, client: crate::Client) -> Result<Self> {
        let pk = client.address(&vault_id, asset)?;
        Ok(Self {
            vault_id,
            asset: asset.to_string(),
            pk,
            client,
            poll_config: PollConfig::default(),
        })
    }

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
