use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TransactPointsRequest {
    /// Target month in YYYY-MM format. Defaults to current month if omitted.
    #[field_doc(en = "Target month in YYYY-MM format. Defaults to current month.", ko = "대상 월 (YYYY-MM 형식). 미입력 시 현재 월.")]
    #[serde(default = "default_month")]
    pub month: String,
    /// Human-readable memo for the transaction.
    #[field_doc(en = "Human-readable memo for the transaction.", ko = "트랜잭션에 대한 메모.")]
    pub description: Option<String>,
    /// Transaction details. Use `tx_type` to select Award, Deduct, Transfer, or Exchange.
    #[field_doc(en = "Transaction type and details. Set tx_type to Award, Deduct, Transfer, or Exchange.", ko = "트랜잭션 유형 및 상세. tx_type을 Award, Deduct, Transfer, Exchange 중 선택.")]
    #[serde(flatten)]
    pub tx: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
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
