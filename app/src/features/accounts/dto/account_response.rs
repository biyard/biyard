use serde::{Deserialize, Serialize};
use crate::common::types::Partition;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountResponse {
    pub pk: Partition,
    pub name: String,
    pub email: String,
    pub created_at: i64,
}
