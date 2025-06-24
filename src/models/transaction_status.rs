use serde::{Deserialize, Serialize};

/// TransactionStatus : The primary status of the transaction.  For details, see [Primary transaction statuses](https://developers.fireblocks.com/reference/primary-transaction-statuses)
/// The primary status of the transaction.  For details, see [Primary transaction statuses](https://developers.fireblocks.com/reference/primary-transaction-statuses)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "PENDING_AML_SCREENING")]
    PendingAmlScreening,
    #[serde(rename = "PENDING_ENRICHMENT")]
    PendingEnrichment,
    #[serde(rename = "PENDING_AUTHORIZATION")]
    PendingAuthorization,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "PENDING_SIGNATURE")]
    PendingSignature,
    #[serde(rename = "PENDING_3RD_PARTY_MANUAL_APPROVAL")]
    Pending3RdPartyManualApproval,
    #[serde(rename = "PENDING_3RD_PARTY")]
    Pending3RdParty,
    #[serde(rename = "BROADCASTING")]
    Broadcasting,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CONFIRMING")]
    Confirming,
    #[serde(rename = "CANCELLING")]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "FAILED")]
    Failed,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Submitted => write!(f, "SUBMITTED"),
            Self::PendingAmlScreening => write!(f, "PENDING_AML_SCREENING"),
            Self::PendingEnrichment => write!(f, "PENDING_ENRICHMENT"),
            Self::PendingAuthorization => write!(f, "PENDING_AUTHORIZATION"),
            Self::Queued => write!(f, "QUEUED"),
            Self::PendingSignature => write!(f, "PENDING_SIGNATURE"),
            Self::Pending3RdPartyManualApproval => write!(f, "PENDING_3RD_PARTY_MANUAL_APPROVAL"),
            Self::Pending3RdParty => write!(f, "PENDING_3RD_PARTY"),
            Self::Broadcasting => write!(f, "BROADCASTING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Confirming => write!(f, "CONFIRMING"),
            Self::Cancelling => write!(f, "CANCELLING"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Blocked => write!(f, "BLOCKED"),
            Self::Rejected => write!(f, "REJECTED"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

impl Default for TransactionStatus {
    fn default() -> TransactionStatus {
        Self::Submitted
    }
}
