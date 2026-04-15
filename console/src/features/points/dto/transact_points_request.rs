use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TransactPointsRequest {
    /// Target month in YYYY-MM format. Defaults to current month if omitted.
    #[serde(default = "default_month")]
    pub month: String,
    /// Human-readable memo for the transaction.
    pub description: Option<String>,
    /// Transaction details. Use `tx_type` to select Award, Deduct, Transfer, or Exchange.
    #[serde(flatten)]
    pub tx: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(tag = "tx_type")]
pub enum Transaction {
    /// Award points to a user.
    Award {
        /// Target user identifier.
        to: String,
        /// Number of points to award. Must be positive.
        amount: i64,
    },
    /// Deduct points from a user.
    Deduct {
        /// User to deduct from.
        from: String,
        /// Number of points to deduct. Must be positive.
        amount: i64,
    },
    /// Transfer points between two users.
    Transfer {
        /// User to transfer from.
        from: String,
        /// User to transfer to.
        to: String,
        /// Number of points to transfer. Must be positive.
        amount: i64,
    },
    /// Exchange points for tokens (must go through mint_token endpoint).
    Exchange {
        /// User exchanging points.
        from: String,
        /// Number of points to exchange.
        amount: i64,
    },
}

fn default_month() -> String {
    crate::common::utils::time_utils::timestamp_to_yyyy_mm()
}
