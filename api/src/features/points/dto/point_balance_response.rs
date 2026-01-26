use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct PointBalanceResponse {
    #[schemars(description = "Project ID")]
    pub project_id: Partition,

    #[schemars(description = "Meta user ID")]
    pub meta_user_id: String,

    #[schemars(description = "Month in YYYY-MM format")]
    pub month: String,

    #[schemars(description = "Current balance")]
    pub balance: i64,

    #[schemars(description = "Total earned this month")]
    pub total_earned: i64,

    #[schemars(description = "Total spent this month")]
    pub total_spent: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,

    #[schemars(description = "Total points distributed to all users this month (project-level)")]
    #[serde(default)]
    pub project_total_points: i64,

    #[schemars(description = "Total tokens to be distributed this month")]
    #[serde(default)]
    pub monthly_token_supply: i64,
}

impl From<crate::features::points::PointBalance> for PointBalanceResponse {
    fn from(balance: crate::features::points::PointBalance) -> Self {
        Self {
            project_id: balance.project_id,
            meta_user_id: balance.meta_user_id,
            month: balance.month,
            balance: balance.balance,
            total_earned: balance.total_earned,
            total_spent: balance.total_spent,
            updated_at: balance.updated_at,
            project_total_points: 0,
            monthly_token_supply: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct PointBalancesResponse {
    #[schemars(description = "List of point balances by month")]
    pub balances: Vec<PointBalanceResponse>,

    #[schemars(description = "Total balance across all months")]
    pub total_balance: i64,
}
