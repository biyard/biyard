use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTokenRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub amount: i64,
    pub description: Option<String>,
}
