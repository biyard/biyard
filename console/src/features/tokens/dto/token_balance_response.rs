use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TokenBalanceResponse {
    /// Project identifier.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub project_id: Partition,
    /// User identifier.
    pub meta_user_id: String,
    /// Token balance in smallest units.
    pub balance: i64,
    /// Latest mint transaction hash.
    pub tx_hash: Option<String>,
    /// Creation timestamp (Unix epoch seconds).
    pub created_at: i64,
    /// Last update timestamp (Unix epoch seconds).
    pub updated_at: i64,
}
