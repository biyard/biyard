use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactPointsResponse {
    pub transaction_id: String,
    pub month: String,
    pub meta_user_id: String,
    pub transaction_type: String,
    pub amount: i64,
}
