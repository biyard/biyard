use crate::common::types::Partition;
use crate::features::points::TransactionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PointTransactionResponse {
    pub project_id: Partition,
    pub meta_user_id: String,
    pub month: String,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub target_user_id: Option<String>,
    pub description: Option<String>,
    pub created_at: i64,
}
