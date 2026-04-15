use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MonthlySummaryItem {
    /// Month in YYYY-MM format.
    pub month: String,
    /// Total points earned by the user this month.
    pub total_earned: i64,
    /// Total points spent by the user this month.
    pub total_spent: i64,
    /// Net point balance for this month.
    pub balance: i64,
    /// Total points across all users in the project for this month.
    pub project_total_points: i64,
    /// Monthly token supply configured for the project.
    pub monthly_token_supply: i64,
    /// Whether the user has already exchanged points for tokens this month.
    pub exchanged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MonthlySummariesResponse {
    /// List of monthly summaries, ordered chronologically.
    pub months: Vec<MonthlySummaryItem>,
}
