use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferPeerPathType {
    #[serde(rename = "VAULT_ACCOUNT")]
    VaultAccount,
}

impl std::fmt::Display for TransferPeerPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::VaultAccount => write!(f, "VAULT_ACCOUNT"),
        }
    }
}

impl Default for TransferPeerPathType {
    fn default() -> TransferPeerPathType {
        Self::VaultAccount
    }
}
