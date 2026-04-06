use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimResponse {
    pub tx_hash: String,
    pub to: String,
    pub amount: u64,
    pub chain_id: u64,
}
