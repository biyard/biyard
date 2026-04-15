use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PointBalanceResponse {
    /// Project identifier.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub project_id: Partition,
    /// User identifier.
    pub meta_user_id: String,
    /// Month in YYYY-MM format.
    pub month: String,
    /// Current point balance for this user in this month.
    pub balance: i64,
    /// Total points earned this month.
    pub total_earned: i64,
    /// Total points spent this month.
    pub total_spent: i64,
    /// Last update timestamp (Unix epoch seconds).
    pub updated_at: i64,
    /// Total points across all users in the project for this month.
    #[serde(default)]
    pub project_total_points: i64,
    /// Monthly token supply configured for the project.
    #[serde(default)]
    pub monthly_token_supply: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointBalancesResponse {
    pub balances: Vec<PointBalanceResponse>,
    pub total_balance: i64,
}
