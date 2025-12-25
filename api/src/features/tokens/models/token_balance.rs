use crate::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct TokenBalance {
    #[schemars(description = "Project ID (pk)")]
    pub pk: Partition,

    #[schemars(description = "User entity type (USER#<meta_user_id>)")]
    pub sk: EntityType,

    #[schemars(description = "Meta user ID")]
    pub meta_user_id: String,

    #[schemars(description = "Current balance")]
    pub balance: i64,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl TokenBalance {
    pub fn new(project_id: Partition, meta_user_id: String) -> Self {
        let now = time_utils::get_now();

        Self {
            pk: project_id,
            sk: EntityType::User(meta_user_id.clone()),
            meta_user_id,
            balance: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(project_id: ProjectPartition, meta_user_id: String) -> (Partition, EntityType) {
        (project_id.into(), EntityType::User(meta_user_id))
    }

    pub fn add_tokens(&mut self, amount: i64) {
        self.balance += amount;
        self.updated_at = time_utils::get_now();
    }

    pub fn deduct_tokens(&mut self, amount: i64) -> Result<()> {
        if self.balance < amount {
            return Err(Error::InsufficientTokens);
        }
        self.balance -= amount;
        self.updated_at = time_utils::get_now();
        Ok(())
    }
}
