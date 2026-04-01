use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MintTokenRequest {
    pub meta_user_id: String,
    pub amount: i64,
    pub description: Option<String>,
}
