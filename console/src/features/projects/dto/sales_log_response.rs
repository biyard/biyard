use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SalesLogResponse {
    pub id: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub created_at: i64,
}
