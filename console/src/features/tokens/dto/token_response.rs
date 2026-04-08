use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenResponse {
    pub pk: Partition,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub circulating_supply: i64,
    pub description: Option<String>,
    pub contract_address: Option<String>,
    pub treasury_contract_address: Option<String>,
    pub stable_token_address: Option<String>,
    pub chain_id: Option<u64>,
    pub deployment_tx_hash: Option<String>,
    pub treasury_deployment_tx_hash: Option<String>,
    pub treasury_reserve_bps: u64,
    pub created_at: i64,
    pub updated_at: i64,
}
