use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResponse {
    pub pk: Partition,
    pub name: String,
    pub email: String,
    pub created_at: i64,
}
