use crate::common::types::Partition;
use crate::features::points::TransactionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PointTransactionResponse {
    /// Project identifier.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub project_id: Partition,
    /// User identifier involved in this transaction.
    pub meta_user_id: String,
    /// Month in YYYY-MM format.
    pub month: String,
    /// Transaction type: Award, Deduct, Transfer, or Exchange.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub transaction_type: TransactionType,
    /// Point amount. Negative for outgoing transfers/deductions.
    pub amount: i64,
    /// Counterparty user ID (for Transfer type).
    pub target_user_id: Option<String>,
    /// Human-readable memo.
    pub description: Option<String>,
    /// Creation timestamp (Unix epoch seconds).
    pub created_at: i64,
}
