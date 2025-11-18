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
    #[dynamo(index = "gsi1", prefix = "PB", pk, name = "find_by_project")]
    pub project_id: Partition,

    #[schemars(description = "Month in YYYY-MM format")]
    #[dynamo(index = "gsi1", sk, prefix = "MONTH", name = "find_by_project")]
    #[dynamo(index = "gsi2", sk, name = "find_by_meta_user")]
    pub month: String,

    #[schemars(description = "Meta user ID (customer's user ID)")]
    #[dynamo(index = "gsi2", pk, prefix = "PB", name = "find_by_meta_user")]
    pub meta_user_id: String,

    #[schemars(description = "Current balance for this month")]
    #[serde(default)]
    pub balance: i64,

    #[schemars(description = "Total points earned this month")]
    #[serde(default)]
    pub total_earned: i64,

    #[schemars(description = "Total points spent this month")]
    #[serde(default)]
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

    pub fn keys(
        project_pk: Partition,
        meta_user_id: String,
        month: String,
    ) -> (CompositePartition, EntityType) {
        let pk = (project_pk, Partition::MetaUser(meta_user_id)).into();
        let sk = EntityType::Month(month);
        (pk, sk)
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
