use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMinterResponse {
    pub minter_address: String,
    pub tx_hash: String,
    pub chain_id: u64,
}
