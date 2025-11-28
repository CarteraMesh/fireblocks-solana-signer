use {
    super::*,
    crate::{SOL, SOL_TEST},
    fireblocks_config::FireblocksConfig,
};

impl FireblocksSigner {
    /// Creates a new `FireblocksSigner` from Fireblocks configuration.
    ///
    /// This function initializes a Fireblocks signer using configuration
    /// profiles and a callback for transaction status updates.
    ///
    /// # Feature Flag
    ///
    /// This function is only available when the `config` feature is enabled.
    ///
    /// ```toml
    /// [dependencies]
    /// fireblocks-solana-signer = { version = "1", features = ["config"] }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `profiles` - Slice of profile names to load from configuration.
    ///   Pass an empty slice `&[]` to use default configuration.
    ///   See [`fireblocks-config`](https://docs.rs/fireblocks-config/latest/fireblocks_config/)
    ///   for configuration details.
    /// * `callback` - Callback function that will be called with transaction
    ///   status updates during the signing process.
    ///
    /// # Returns
    ///
    /// Returns a `Result<FireblocksSigner>` on success, or an error if:
    /// - Configuration initialization fails
    /// - Client building fails
    /// - Address resolution fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fireblocks_solana_signer::FireblocksSigner;
    ///
    /// // Create signer with default configuration and custom logging
    /// let signer = FireblocksSigner::try_from_config::<String>(&[], |tx_response| {
    ///     println!("Transaction status: {}", tx_response)
    /// })?;
    ///
    /// // Create signer with specific profiles
    /// let profiles = ["mainnet", "production"];
    /// let signer = FireblocksSigner::try_from_config(&profiles, |tx_response| {
    ///     eprintln!("TX Update: {}", tx_response)
    /// })?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Network Selection
    ///
    /// The function automatically selects the appropriate Solana network:
    /// - Uses `SOL` (mainnet) if `cfg.mainnet` is `true`
    /// - Uses `SOL_TEST` (devnet/testnet) if `cfg.mainnet` is `false`
    ///
    /// # Callback Behavior
    ///
    /// The callback function receives a [`TransactionResponse`] and is called
    /// during the transaction polling process to provide status updates.
    /// The callback must be a function pointer, not a closure that captures
    /// variables from the surrounding scope.
    ///
    /// # Configuration
    ///
    /// This function uses the [`fireblocks-config`](https://docs.rs/fireblocks-config/latest/fireblocks_config/)
    /// crate for loading Fireblocks API credentials and settings from
    /// configuration files.
    pub fn try_from_config<S>(
        profiles: &[S],
        callback: fn(&crate::TransactionResponse),
    ) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let cfg = FireblocksConfig::init_with_profiles(profiles)?;
        let asset = if cfg.mainnet { SOL } else { SOL_TEST };
        let pk: Option<String> = cfg.get_extra("solana_pub_key").ok();
        let builder = ClientBuilder::new(&cfg.api_key, &cfg.get_key()?)
            .with_url(&cfg.url)
            .with_timeout(Duration::from_secs(10))
            .with_connect_timeout(Duration::from_secs(7));
        let (client, pk) = crate::build_client_and_address_blocking_safe(
            builder,
            cfg.signer.vault.clone(),
            asset.clone(),
            pk,
        )?;

        Ok(FireblocksSigner::builder()
            .broadcast(false)
            .pk(pk)
            .client(client)
            .asset(asset)
            .vault_id(cfg.signer.vault)
            .poll_config(
                PollConfig::builder()
                    .timeout(cfg.signer.poll_timeout)
                    .interval(cfg.signer.poll_interval)
                    .callback(callback)
                    .build(),
            )
            .build())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_config() -> anyhow::Result<()> {
        if std::env::var("CI").ok().is_none() {
            eprintln!("skipping config test, not in CI");
            return Ok(());
        }
        FireblocksSigner::try_from_config(&["default"], |tx| tracing::info!("{tx}"))?;
        Ok(())
    }
}
