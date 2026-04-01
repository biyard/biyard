use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalanceResponse {
    pub project_id: Partition,
    pub meta_user_id: String,
    pub balance: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
