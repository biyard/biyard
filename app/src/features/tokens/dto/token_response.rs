use serde::{Deserialize, Serialize};
use crate::common::types::Partition;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub pk: Partition,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i64,
    pub circulating_supply: i64,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
