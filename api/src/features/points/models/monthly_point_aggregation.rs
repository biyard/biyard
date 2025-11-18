use crate::{utils::time_utils::timestamp_to_yyyy_mm, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo, DynamoEntity)]
pub struct MonthlyPointAggregation {
    pub pk: CompositePartition<ProjectPartition, MetaUserPartition>,
    pub sk: EntityType,

    pub supplied_points: i64,
    pub traded_points: i64,
    pub awarded_points: i64,
    pub deducted_points: i64,
    pub exchanged_points: i64,

    pub updated_at: i64,
}

impl MonthlyPointAggregation {
    pub fn new(project_pk: ProjectPartition, meta_user_pk: MetaUserPartition) -> Self {
        Self {
            pk: CompositePartition(project_pk, meta_user_pk),
            sk: EntityType::MonthlyPointAggregation(timestamp_to_yyyy_mm()),
            supplied_points: 0,
            traded_points: 0,
            awarded_points: 0,
            deducted_points: 0,
            exchanged_points: 0,
            updated_at: time_utils::get_now(),
        }
    }

    pub fn keys(
        project_pk: ProjectPartition,
        meta_user_pk: MetaUserPartition,
        date: String,
    ) -> (
        CompositePartition<ProjectPartition, MetaUserPartition>,
        EntityType,
    ) {
        (
            CompositePartition(project_pk, meta_user_pk),
            EntityType::MonthlyPointAggregation(date),
        )
    }
}
