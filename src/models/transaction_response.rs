use {
    crate::models,
    serde::{Deserialize, Serialize},
    solana_signature::Signature,
    std::fmt::Display,
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Fireblocks Transaction ID
    #[serde(rename = "id")]
    pub id: String,
    /// Unique externbal transaction identifier provided by the user. Fireblocks
    /// highly recommends setting an `externalTxId` for every transaction
    /// created, to avoid submitting the same transaction twice.
    #[serde(rename = "externalTxId", skip_serializing_if = "Option::is_none")]
    pub external_tx_id: Option<String>,
    #[serde(rename = "status")]
    pub status: models::TransactionStatus,
    #[serde(rename = "subStatus", skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<models::TransactionSubStatus>,
    /// The hash of the transaction on the blockchain.  * This parameter exists
    /// if at least one of the following conditions is met:       1. The
    /// transaction’s source type is `UNKNOWN`, `WHITELISTED_ADDRESS`,
    /// `NETWORK_CONNECTION`, `ONE_TIME_ADDRESS`, `FIAT_ACCOUNT` or
    /// `GAS_STATION`.       2. The transaction’s source type is `VAULT` and the
    /// status is either: `CONFIRMING`, `COMPLETED`, or was in any of these
    /// statuses prior to changing to `FAILED` or `REJECTED`. In some instances,
    /// transactions in status `BROADCASTING` will include the txHash as well.
    /// 3. The transaction’s source type is `EXCHANGE_ACCOUNT` and the
    /// transaction’s destination type is `VAULT`, and the status is either:
    /// `CONFIRMING`, `COMPLETED`, or was in any of these status prior to
    /// changing to `FAILED`.   * In addition, the following conditions must be
    /// met:      1. The asset is a crypto asset (not fiat).      2. The
    /// transaction operation is not `RAW` or `TYPED_MESSAGE`.
    #[serde(rename = "txHash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Custom note, not sent to the blockchain, that describes the transaction
    /// at your Fireblocks workspace.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The ID of the asset for `TRANSFER`, `MINT`, `BURN`, `ENABLE_ASSET`,`STAKE` ,`UNSTAKE` or `WITHDRAW` operations. [See the list of supported assets and their IDs on Fireblocks.](https://developers.fireblocks.com/reference/getsupportedassets)
    #[serde(rename = "assetId")]
    pub asset_id: String,
    // #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    // pub source: Option<models::SourceTransferPeerPathResponse>,
    /// For account based assets only, the source address of the transaction.
    /// **Note:** If the status is `CONFIRMING`, `COMPLETED`, or has been
    /// `CONFIRMING`; then moved forward to `FAILED` or `REJECTED`, then this
    /// parameter will contain the source address. In any other case, this
    /// parameter will be empty.
    #[serde(rename = "sourceAddress", skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    /// Source address tag for Tag/Memo supporting assets, or Bank Transfer
    /// Description for the fiat provider BLINC (by BCB Group).
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // #[serde(rename = "amountInfo", skip_serializing_if = "Option::is_none")]
    // pub amount_info: Option<models::AmountInfo>,
    /// The transaction’s creation date and time, in unix timestamp.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    /// The transaction’s last update date and time, in unix timestamp.
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<u64>,
    /// User ID of the initiator of the transaction.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// User ID’s of the signers of the transaction.
    #[serde(rename = "signedBy", skip_serializing_if = "Option::is_none")]
    pub signed_by: Option<Vec<String>>,
    /// User ID of the user that rejected the transaction (in case it was
    /// rejected).
    #[serde(rename = "rejectedBy", skip_serializing_if = "Option::is_none")]
    pub rejected_by: Option<String>,
    #[serde(rename = "customerRefId", skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
    // #[serde(rename = "extraParameters", skip_serializing_if = "Option::is_none")]
    // pub extra_parameters: Option<models::ExtraParameters>,
    /// An array of signed messages
    // #[serde(rename = "signedMessages", skip_serializing_if = "Option::is_none")]
    // pub signed_messages: Option<Vec<models::SignedMessage>>,
    /// The number of confirmations of the transaction. The number will increase
    /// until the transaction will be considered completed according to the
    /// confirmation policy.
    #[serde(rename = "numOfConfirmations", skip_serializing_if = "Option::is_none")]
    pub num_of_confirmations: Option<i32>,
    #[serde(rename = "systemMessages", skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<models::SystemMessageInfo>,
    /// `subStatus` =  'SMART_CONTRACT_EXECUTION_FAILED'.
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl Display for TransactionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hash = String::from("N/A");
        if self.tx_hash.is_some() {
            hash = self.tx_hash.clone().unwrap_or_default();
        }
        write!(
            f,
            "txid: {} status: {} hash: {}",
            self.id, self.status, hash
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressType {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "WHITELISTED")]
    Whitelisted,
    #[serde(rename = "ONE_TIME")]
    OneTime,
}

impl Default for AddressType {
    fn default() -> AddressType {
        Self::Empty
    }
}

impl TryFrom<TransactionResponse> for Signature {
    type Error = crate::Error;

    fn try_from(response: TransactionResponse) -> Result<Self, Self::Error> {
        match response.tx_hash {
            Some(hash) => hash.parse().map_err(|_| {
                crate::Error::InvalidMessage(format!("Invalid signature format: {hash}"))
            }),
            None => Err(crate::Error::InvalidMessage(
                "Transaction response does not contain a tx_hash".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::models::TransactionStatus};

    #[test]
    fn test_transaction_response_default() {
        let default_response = TransactionResponse::default();

        assert_eq!(default_response.id, String::default());
        assert_eq!(default_response.external_tx_id, None);
        assert_eq!(default_response.status, TransactionStatus::default());
        assert_eq!(default_response.sub_status, None);
        assert_eq!(default_response.tx_hash, None);
        assert_eq!(default_response.note, None);
        assert_eq!(default_response.asset_id, String::default());
        assert_eq!(default_response.source_address, None);
        assert_eq!(default_response.tag, None);
        assert_eq!(default_response.created_at, None);
        assert_eq!(default_response.last_updated, None);
        assert_eq!(default_response.created_by, None);
        assert_eq!(default_response.signed_by, None);
        assert_eq!(default_response.rejected_by, None);
        assert_eq!(default_response.customer_ref_id, None);
        assert_eq!(default_response.num_of_confirmations, None);
        assert_eq!(default_response.system_messages, None);
        assert_eq!(default_response.error_description, None);
    }

    #[test]
    fn test_address_type_default() {
        let default_address_type = AddressType::default();
        assert_eq!(default_address_type, AddressType::Empty);
    }

    #[test]
    fn test_try_from_transaction_response_success() {
        // Test successful conversion with valid signature
        let response = TransactionResponse {
        tx_hash: Some("5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW".to_string()),
        ..Default::default()
        };

        let signature_result: Result<Signature, crate::Error> = response.try_into();
        assert!(signature_result.is_ok());
    }

    #[test]
    fn test_try_from_transaction_response_no_hash() {
        // Test error when tx_hash is None
        let response = TransactionResponse::default(); // tx_hash is None by default

        let signature_result: Result<Signature, crate::Error> = response.try_into();
        assert!(signature_result.is_err());

        if let Err(crate::Error::InvalidMessage(msg)) = signature_result {
            assert_eq!(msg, "Transaction response does not contain a tx_hash");
        } else {
            panic!("Expected InvalidMessage error for missing tx_hash");
        }
    }

    #[test]
    fn test_try_from_transaction_response_invalid_signature() {
        // Test error when tx_hash contains invalid signature format
        let response = TransactionResponse {
            tx_hash: Some("invalid_signature_format".to_string()),
            ..Default::default()
        };

        let signature_result: Result<Signature, crate::Error> = response.try_into();
        assert!(signature_result.is_err());

        if let Err(crate::Error::InvalidMessage(msg)) = signature_result {
            assert_eq!(msg, "Invalid signature format: invalid_signature_format");
        } else {
            panic!("Expected InvalidMessage error for invalid signature format");
        }
    }

    #[test]
    fn test_display_formatting() {
        let mut response = TransactionResponse {
            id: "test_id".to_string(),
            ..Default::default()
        };

        // Test display without tx_hash
        let display_str = format!("{response}");
        assert!(display_str.contains("txid: test_id"));
        assert!(display_str.contains("hash: N/A"));

        // Test display with tx_hash
        response.tx_hash = Some("test_hash".to_string());
        let display_str = format!("{response}");
        assert!(display_str.contains("txid: test_id"));
        assert!(display_str.contains("hash: test_hash"));
    }

    #[test]
    fn test_address_type_variants() {
        // Test all AddressType variants
        assert_eq!(AddressType::Empty, AddressType::default());
        assert_ne!(AddressType::Whitelisted, AddressType::default());
        assert_ne!(AddressType::OneTime, AddressType::default());

        // Test ordering
        assert!(AddressType::Empty < AddressType::Whitelisted);
        assert!(AddressType::Whitelisted < AddressType::OneTime);
    }

    // #[test]
    // fn test_transaction_response_clone() {
    //     let mut original = TransactionResponse::default();
    //     original.id = "test_id".to_string();
    //     original.tx_hash = Some("test_hash".to_string());
    //
    //     let cloned = original.clone();
    //     assert_eq!(original, cloned);
    //     assert_eq!(original.id, cloned.id);
    //     assert_eq!(original.tx_hash, cloned.tx_hash);
    // }
}
