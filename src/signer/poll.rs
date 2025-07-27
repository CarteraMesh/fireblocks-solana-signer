use std::{fmt::Debug, time::Duration};
/// Configuration for polling Fireblocks transaction status.
///
/// This struct controls how the signer polls Fireblocks for transaction
/// completion, including timeout duration, polling interval, and callback
/// function for status updates.
///
/// # Examples
///
/// ```
/// use {fireblocks_solana_signer::PollConfig, std::time::Duration};
///
/// let config = PollConfig::builder()
///     .timeout(Duration::from_secs(30))
///     .interval(Duration::from_secs(2))
///     .callback(|response| println!("Transaction status: {:?}", response))
///     .build();
/// ```
#[derive(Clone, Debug, bon::Builder)]
pub struct PollConfig {
    /// Maximum time to wait for transaction completion.
    ///
    /// If the transaction doesn't complete within this duration, polling will
    /// stop and return a timeout error.
    pub timeout: Duration,

    /// Interval between polling requests.
    ///
    /// This determines how frequently the signer checks the transaction status
    /// with Fireblocks.
    pub interval: Duration,

    /// Callback function called on each polling iteration.
    ///
    /// This function receives the current transaction response and can be used
    /// for logging, monitoring, or other side effects during the polling
    /// process.
    pub callback: fn(&crate::TransactionResponse),
}

impl Default for PollConfig {
    /// Creates a default polling configuration.
    ///
    /// Default values:
    /// - `timeout`: 15 seconds
    /// - `interval`: 5 seconds
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(15),
            interval: Duration::from_secs(5),
            callback: |t| log::info!("{t}"),
        }
    }
}
