use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct MonthlyPointAggregation {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub supplied_points: i64,
}
