use crate::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct PointBalance {
    #[schemars(description = "Composite key: PROJECT#<project_id>#META_USER#<meta_user_id>")]
    pub pk: CompositePartition,
    #[schemars(description = "Sort key: MONTH#<YYYY-MM>")]
    pub sk: EntityType,

    #[schemars(description = "Project ID")]
    #[dynamo(index = "gsi1", pk, name = "find_by_project")]
    pub project_id: Partition,

    #[schemars(description = "Month in YYYY-MM format")]
    #[dynamo(index = "gsi1", sk, prefix = "MONTH", name = "find_by_project")]
    pub month: String,

    #[schemars(description = "Meta user ID (customer's user ID)")]
    pub meta_user_id: String,

    #[schemars(description = "Current balance for this month")]
    pub balance: i64,

    #[schemars(description = "Total points earned this month")]
    pub total_earned: i64,

    #[schemars(description = "Total points spent this month")]
    pub total_spent: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl PointBalance {
    pub fn new(project_pk: Partition, meta_user_id: String) -> Self {
        let now = time_utils::get_now();
        let month = time_utils::timestamp_to_yyyy_mm();

        Self {
            pk: (
                project_pk.clone(),
                Partition::MetaUser(meta_user_id.clone()),
            )
                .into(),
            sk: EntityType::Month(month.clone()),
            project_id: project_pk,
            month: month,
            meta_user_id,
            balance: 0,
            total_earned: 0,
            total_spent: 0,
            updated_at: now,
        }
    }

    pub fn add_points(&mut self, amount: i64) {
        self.balance += amount;
        self.total_earned += amount;
        self.updated_at = time_utils::get_now();
    }

    pub fn deduct_points(&mut self, amount: i64) -> Result<()> {
        if self.balance < amount {
            return Err(Error::InsufficientPoints);
        }
        self.balance -= amount;
        self.total_spent += amount;
        self.updated_at = time_utils::get_now();
        Ok(())
    }
}
