use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    PubkeyError(#[from] solana_pubkey::ParsePubkeyError),

    #[error("Failed to get vault account {0}")]
    FireblocksVaultError(String),

    #[error(transparent)]
    BincodeEncodeError(#[from] bincode::Error),

    #[error("Failed to deserialize solana message {0}")]
    InvalidMessage(String),

    #[error("No signature available {0}")]
    FireblocksNoSig(String),

    #[error("No pubkey for vault {0}")]
    FireblocksNoPubkey(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    JwtError(#[from] crate::jwt::JwtError),

    #[error(transparent)]
    TokenError(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    FireblocksServerError(String),

    #[error(transparent)]
    JsonErr(#[from] serde_json::Error),

    #[error("Operation timed out")]
    Timeout,

    #[error("Solan RPC error {0}")]
    ParseAddressTableError(String),

    #[error("Solan RPC Error {0}")]
    SolanaRpcErrpr(String),

    #[error("pubkey on lookuptable is invalid")]
    InvalidPubkey,
}
