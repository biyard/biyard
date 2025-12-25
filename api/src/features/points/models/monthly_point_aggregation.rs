use crate::{utils::time_utils::timestamp_to_yyyy_mm, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo, DynamoEntity)]
pub struct MonthlyPointAggregation {
    pub pk: CompositePartition<ProjectPartition, Partition>,
    pub sk: EntityType,

    pub supplied_points: i64,
    pub traded_points: i64,
    pub awarded_points: i64,
    pub deducted_points: i64,
    pub exchanged_points: i64,

    pub updated_at: i64,

    #[dynamo(index = "gsi1", pk, prefix = "MPA", name = "find_by_date")]
    pub project_pk: Partition,

    #[dynamo(index = "gsi1", sk, name = "find_by_date")]
    pub date: String,
}

impl MonthlyPointAggregation {
    pub fn new(project_pk: ProjectPartition) -> Self {
        Self {
            pk: CompositePartition(
                project_pk.clone(),
                Partition::MonthlyPoints(timestamp_to_yyyy_mm()),
            ),
            sk: EntityType::MonthlyPointAggregation,
            project_pk: project_pk.into(),
            supplied_points: 0,
            traded_points: 0,
            awarded_points: 0,
            deducted_points: 0,
            exchanged_points: 0,
            updated_at: time_utils::get_now(),
            date: timestamp_to_yyyy_mm(),
        }
    }

    pub fn keys(
        project_pk: ProjectPartition,
        date: String,
    ) -> (
        CompositePartition<ProjectPartition, MonthlyPointsPartition>,
        EntityType,
    ) {
        (
            CompositePartition(project_pk, MonthlyPointsPartition(date)),
            EntityType::MonthlyPointAggregation,
        )
    }
}
