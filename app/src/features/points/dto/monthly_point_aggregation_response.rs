use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MonthlyPointAggregationResponse {
    pub date: String,
    pub supplied_points: i64,
    pub traded_points: i64,
    pub awarded_points: i64,
    pub deducted_points: i64,
    pub exchanged_points: i64,
}
