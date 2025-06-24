use {
    super::transfer_peer_path_type::TransferPeerPathType,
    serde::{Deserialize, Serialize},
};

/// SourceTransferPeerPath : The source of the transaction.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTransferPeerPath {
    #[serde(rename = "type")]
    pub r#type: TransferPeerPathType,
    #[serde(rename = "id")]
    pub id: String,
}

impl SourceTransferPeerPath {
    /// The source of the transaction.
    pub fn new(id: String) -> SourceTransferPeerPath {
        SourceTransferPeerPath {
            r#type: TransferPeerPathType::VaultAccount,
            id,
        }
    }
}
