use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySummaryItem {
    pub month: String,
    pub total_earned: i64,
    pub total_spent: i64,
    pub balance: i64,
    pub project_total_points: i64,
    pub monthly_token_supply: i64,
    pub exchanged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySummariesResponse {
    pub months: Vec<MonthlySummaryItem>,
}
