use {std::sync::mpsc::RecvError, thiserror::Error};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    PubkeyError(#[from] solana_pubkey::ParsePubkeyError),

    #[error(transparent)]
    BincodeEncodeError(#[from] bincode::Error),

    #[error("Failed to deserialize solana message {0}")]
    InvalidMessage(String),

    #[error("No signature available {0}")]
    FireblocksNoSig(String),

    #[error("No pubkey for vault {0}")]
    FireblocksNoPubkey(String),

    #[error(transparent)]
    ChannelRecvError(#[from] RecvError),

    #[error(transparent)]
    SignatureError(#[from] solana_signature::ParseSignatureError),

    #[error(transparent)]
    FireblocksClientError(#[from] fireblocks_signer_transport::FireblocksClientError),

    #[error("{0}")]
    JsonParseErr(String),

    #[error(transparent)]
    JsonErr(#[from] serde_json::Error),

    #[error("Operation timed out {0}")]
    Timeout(String),

    #[error("{0}")]
    ThreadPanic(String),

    #[error("{0}")]
    ChannelClosed(String),

    #[error("Solan RPC error {0}")]
    ParseAddressTableError(String),

    #[error("Solan RPC Error {0}")]
    SolanaRpcError(String),

    #[error("pubkey on lookuptable is invalid")]
    InvalidPubkey,

    #[error("ENV {0} is missing")]
    EnvMissing(String),

    #[error("Unknown asset {0}")]
    UnknownAsset(String),

    #[error("Tokio join error: {0}")]
    JoinError(String),

    #[error(transparent)]
    ConfigError(#[from] fireblocks_config::Error),
}
