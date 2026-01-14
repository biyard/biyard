use crate::{utils::time_utils::timestamp_to_yyyy_mm, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo, DynamoEntity)]
pub struct MonthlyPointAggregation {
    pub pk: CompositePartition<ProjectPartition, MonthlyPointsPartition>,
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
    pub fn new(project_pk: ProjectPartition) -> Self {
        let now = time_utils::get_now();
        let time = timestamp_to_yyyy_mm();
        let (pk, sk) = Self::keys(project_pk.clone(), time.clone());
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
