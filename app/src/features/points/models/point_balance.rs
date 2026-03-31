use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct PointBalance {
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", prefix = "PB", pk, name = "find_by_project")]
    pub project_id: Partition,

    #[dynamo(index = "gsi1", sk, prefix = "MONTH", name = "find_by_project")]
    #[dynamo(index = "gsi2", sk, name = "find_by_meta_user")]
    pub month: String,

    #[dynamo(index = "gsi2", pk, prefix = "PB", name = "find_by_meta_user")]
    pub meta_user_id: String,

    #[serde(default)]
    pub balance: i64,
    #[serde(default)]
    pub total_earned: i64,
    #[serde(default)]
    pub total_spent: i64,
    pub updated_at: i64,
}

impl PointBalance {
    pub fn new(project_pk: Partition, meta_user_id: String) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let month = crate::common::utils::time_utils::timestamp_to_yyyy_mm();

        Self {
            pk: (
                project_pk.clone(),
                Partition::MetaUser(meta_user_id.clone()),
            )
                .into(),
            sk: EntityType::Month(month.clone()),
            project_id: project_pk,
            month,
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
        self.updated_at = crate::common::utils::time_utils::get_now();
    }

    pub fn deduct_points(&mut self, amount: i64) -> crate::common::Result<()> {
        if self.balance < amount {
            return Err(crate::features::points::PointError::InsufficientPoints.into());
        }
        self.balance -= amount;
        self.total_spent += amount;
        self.updated_at = crate::common::utils::time_utils::get_now();
        Ok(())
    }
}

impl From<PointBalance> for crate::features::points::PointBalanceResponse {
    fn from(balance: PointBalance) -> Self {
        Self {
            project_id: balance.project_id,
            meta_user_id: balance.meta_user_id,
            month: balance.month,
            balance: balance.balance,
            total_earned: balance.total_earned,
            total_spent: balance.total_spent,
            updated_at: balance.updated_at,
            project_total_points: 0,
            monthly_token_supply: 0,
        }
    }
}
