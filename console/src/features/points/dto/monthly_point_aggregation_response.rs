use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MonthlyPointAggregationResponse {
    /// Month in YYYY-MM format.
    pub date: String,
    /// Total points currently in circulation for this month.
    pub supplied_points: i64,
    /// Total points transferred between users this month.
    pub traded_points: i64,
    /// Total points awarded to users this month.
    pub awarded_points: i64,
    /// Total points deducted from users this month.
    pub deducted_points: i64,
    /// Total points exchanged for tokens this month.
    pub exchanged_points: i64,
}

impl MonthlyPointAggregationResponse {
    /// Zero-filled aggregation for a given month. Used when no aggregation
    /// row exists yet (fresh brand) so the API can return `200 OK` instead
    /// of `404`, keeping "no activity" out of the browser console error log.
    pub fn empty(date: String) -> Self {
        Self {
            date,
            ..Self::default()
        }
    }
}
