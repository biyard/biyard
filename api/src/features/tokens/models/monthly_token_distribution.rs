use crate::*;

/// MonthlyTokenDistribution represents monthly token supply for a project.
/// Each project can have different token distribution amounts per month.
/// pk: PROJECT#<project_id>, sk: MONTH#YYYY-MM
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo, DynamoEntity)]
pub struct MonthlyTokenDistribution {
    #[schemars(description = "Project ID (pk)")]
    pub pk: Partition,

    #[schemars(description = "Month entity type (MONTH#YYYY-MM)")]
    pub sk: EntityType,

    #[schemars(description = "Amount of tokens to distribute this month")]
    pub supply_amount: i64,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl MonthlyTokenDistribution {
    pub fn new(project_id: Partition, month: String, supply_amount: i64) -> Self {
        let now = time_utils::get_now();

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
