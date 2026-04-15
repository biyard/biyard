use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TransactPointsResponse {
    /// Unique identifier for this transaction.
    pub transaction_id: String,
    /// Month the transaction was recorded in (YYYY-MM).
    pub month: String,
    /// User identifier involved in the transaction.
    pub meta_user_id: String,
    /// Type of transaction: Award, Deduct, Transfer, or Exchange.
    pub transaction_type: String,
    /// Point amount. Negative for outgoing transfers/deductions.
    pub amount: i64,
}
