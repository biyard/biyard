use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
pub struct MonthlyPointAggregation {
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[serde(default)]
    pub supplied_points: i64,
    #[serde(default)]
    pub traded_points: i64,
    #[serde(default)]
    pub awarded_points: i64,
    #[serde(default)]
    pub deducted_points: i64,
    #[serde(default)]
    pub exchanged_points: i64,
    #[serde(default)]
    pub updated_at: i64,
}

impl MonthlyPointAggregation {
    pub fn new(project_pk: Partition) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let time = crate::common::utils::time_utils::timestamp_to_yyyy_mm();
        let (pk, sk) = Self::keys(project_pk, time);
        Self {
            pk,
            sk,
            supplied_points: 0,
            traded_points: 0,
            awarded_points: 0,
            deducted_points: 0,
            exchanged_points: 0,
            updated_at: now,
        }
    }

    pub fn keys(project_pk: Partition, date: String) -> (CompositePartition, EntityType) {
        (
            CompositePartition(project_pk, Partition::MonthlyPoints(date)),
            EntityType::MonthlyPointAggregation,
        )
    }
}

impl From<MonthlyPointAggregation> for crate::features::points::MonthlyPointAggregationResponse {
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
