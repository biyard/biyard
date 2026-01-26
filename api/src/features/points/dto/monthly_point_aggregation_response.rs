use crate::{features::points::MonthlyPointAggregation, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct MonthlyPointAggregationResponse {
    pub date: String,

    pub supplied_points: i64,
    pub traded_points: i64,
    pub awarded_points: i64,
    pub deducted_points: i64,
    pub exchanged_points: i64,
}

impl From<MonthlyPointAggregation> for MonthlyPointAggregationResponse {
    fn from(aggregation: MonthlyPointAggregation) -> Self {
        let date = aggregation.pk.1.to_string();
        Self {
            date,
            supplied_points: aggregation.supplied_points,
            traded_points: aggregation.traded_points,
            awarded_points: aggregation.awarded_points,
            deducted_points: aggregation.deducted_points,
            exchanged_points: aggregation.exchanged_points,
        }
    }
}
