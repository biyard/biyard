use crate::common::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EnterpriseResponse {
    pub id: String,
    pub pk: Partition,
    pub owner_account_id: Partition,
    pub name: String,
    pub slug: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl EnterpriseResponse {
    pub fn display_name(&self) -> &str {
        &self.name
    }
}
