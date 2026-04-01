use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
pub struct MonthlyTokenDistribution {
    pub pk: Partition,
    pub sk: EntityType,

    pub supply_amount: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl MonthlyTokenDistribution {
    pub fn new(project_id: Partition, month: String, supply_amount: i64) -> Self {
        let now = crate::common::utils::time_utils::get_now();

        Self {
            pk: project_id,
            sk: EntityType::Month(month),
            supply_amount,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(project_id: Partition, month: String) -> (Partition, EntityType) {
        (project_id, EntityType::Month(month))
    }
}
