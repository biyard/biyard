use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResponse {
    pub pk: Partition,
    pub name: String,
    pub email: String,
    pub created_at: i64,
}

impl AccountResponse {
    pub fn id(&self) -> String {
        match &self.pk {
            Partition::Account(uid) => uid.clone(),
            _ => "".to_string(),
        }
    }
}
