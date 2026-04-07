use crate::common::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EnterpriseResponse {
    pub id: String,
    pub pk: Partition,
    pub owner_account_id: Partition,
    pub name: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
}

impl EnterpriseResponse {
    pub fn display_name(&self) -> &str {
        &self.name
    }
}
