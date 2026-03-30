use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactPointsRequest {
    #[serde(default = "default_month")]
    pub month: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub tx: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tx_type")]
pub enum Transaction {
    Award { to: String, amount: i64 },
    Deduct { from: String, amount: i64 },
    Transfer { from: String, to: String, amount: i64 },
    Exchange { from: String, amount: i64 },
}

fn default_month() -> String {
    crate::common::utils::time_utils::timestamp_to_yyyy_mm()
}
