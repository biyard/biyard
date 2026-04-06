use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MintDataResponse {
    pub contract_address: String,
    pub chain_id: u64,
    pub calldata: String,
    pub to: String,
    pub amount: u64,
}
