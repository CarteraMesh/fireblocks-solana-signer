use {
    super::ExtraParameters,
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "operation")]
    pub operation: models::TransactionOperation,
    #[serde(rename = "externalTxId", skip_serializing_if = "Option::is_none")]
    pub external_tx_id: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "source")]
    pub source: models::SourceTransferPeerPath,
    #[serde(rename = "feeLevel")]
    pub fee_level: FeeLevel,
    // #[serde(rename = "priorityFee", skip_serializing_if = "Option::is_none")]
    // pub priority_fee: Option<models::TransactionRequestPriorityFee>,
    /// When set to `true`, in case the current `MEDIUM` fee level is higher
    /// than the one specified in the transaction, the transaction will fail to
    /// avoid getting stuck with no confirmations.
    #[serde(rename = "failOnLowFee")]
    pub fail_on_low_fee: bool,
    #[serde(rename = "extraParameters")]
    pub extra_parameters: models::ExtraParameters,
    #[serde(rename = "customerRefId", skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
}

impl TransactionRequest {
    pub fn new(
        asset_id: String,
        source: models::SourceTransferPeerPath,
        extra_parameters: ExtraParameters,
    ) -> Self {
        Self {
            operation: models::TransactionOperation::ProgramCall,
            note: None,
            asset_id,
            source,
            fee_level: FeeLevel::default(),
            fail_on_low_fee: false,
            // priority_fee: None,
            extra_parameters,
            customer_ref_id: None,
            external_tx_id: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeeLevel {
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

impl Default for FeeLevel {
    fn default() -> FeeLevel {
        Self::Low
    }
}
