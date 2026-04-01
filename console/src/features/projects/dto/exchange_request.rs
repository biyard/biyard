use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExchangeType {
    PointToToken,
    TokenToPoint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRequest {
    pub meta_user_id: String,
    pub exchange_type: ExchangeType,
    pub amount: i64,
    pub month: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeResponse {
    pub amount_exchanged: i64,
    pub amount_received: i64,
    pub exchange_rate: f64,
    pub remaining_balance: i64,
}
