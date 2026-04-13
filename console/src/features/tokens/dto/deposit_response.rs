use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositResponse {
    pub tx_hash: String,
    pub amount: String,
}
