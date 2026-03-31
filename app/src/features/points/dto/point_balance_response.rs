use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PointBalanceResponse {
    pub project_id: Partition,
    pub meta_user_id: String,
    pub month: String,
    pub balance: i64,
    pub total_earned: i64,
    pub total_spent: i64,
    pub updated_at: i64,
    #[serde(default)]
    pub project_total_points: i64,
    #[serde(default)]
    pub monthly_token_supply: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointBalancesResponse {
    pub balances: Vec<PointBalanceResponse>,
    pub total_balance: i64,
}
