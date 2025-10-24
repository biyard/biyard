use crate::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct TokenBalance {
    #[schemars(description = "Composite key: TOKEN#<token_id>#USER#<meta_user_id>")]
    pub pk: Partition,
    #[schemars(description = "Entity type")]
    pub sk: EntityType,

    #[schemars(description = "Token ID")]
    #[dynamo(index = "gsi1", pk, name = "find_by_token")]
    pub token_id: Partition,

    #[schemars(description = "Meta user ID")]
    #[dynamo(index = "gsi1", sk, prefix = "USER", name = "find_by_token")]
    pub meta_user_id: String,

    #[schemars(description = "Project ID")]
    #[dynamo(index = "gsi2", pk, name = "find_by_project_and_user")]
    pub project_id: Partition,

    #[schemars(description = "User ID for GSI2")]
    #[dynamo(index = "gsi2", sk, prefix = "USER", name = "find_by_project_and_user")]
    pub gsi2_sk: String,

    #[schemars(description = "Current balance")]
    pub balance: i64,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl TokenBalance {
    pub fn new(token_id: Partition, project_id: Partition, meta_user_id: String) -> Self {
        let now = time_utils::get_now();
        let pk_value = format!("{}#USER#{}", token_id.to_string(), meta_user_id);
        let gsi2_sk = format!("USER#{}", meta_user_id);

        Self {
            pk: Partition::TokenBalance(pk_value),
            sk: EntityType::TokenBalance,
            token_id,
            meta_user_id: meta_user_id.clone(),
            project_id,
            gsi2_sk,
            balance: 0,
            created_at: now,
            updated_at: now,
        }
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
