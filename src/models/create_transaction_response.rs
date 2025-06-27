use {
    crate::models,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTransactionResponse {
    /// Unique Fireblocks identifier of the transaction
    #[serde(rename = "id")]
    pub id: String,
    /// The primary status of the transaction. For details, see [Primary transaction statuses.](https://developers.fireblocks.com/reference/primary-transaction-statuses)
    #[serde(rename = "status")]
    pub status: models::TransactionStatus,
    #[serde(rename = "systemMessages", skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<models::SystemMessageInfo>,
}

impl Display for CreateTransactionResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.id)
    }
}
