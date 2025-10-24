use crate::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct ProjectToken {
    #[schemars(description = "Token ID")]
    pub pk: Partition,
    #[schemars(description = "Entity type")]
    pub sk: EntityType,

    #[schemars(description = "Project ID that owns this token")]
    #[dynamo(index = "gsi1", pk, name = "find_by_project")]
    pub project_id: Partition,

    #[schemars(description = "GSI1 sort key (EntityType)")]
    #[dynamo(index = "gsi1", sk, name = "find_by_project")]
    pub gsi1_sk: EntityType,

    #[schemars(description = "Token name")]
    pub name: String,

    #[schemars(description = "Token symbol")]
    pub symbol: String,

    #[schemars(description = "Number of decimals")]
    pub decimals: u8,

    #[schemars(description = "Total supply")]
    pub total_supply: i64,

    #[schemars(description = "Circulating supply")]
    pub circulating_supply: i64,

    #[schemars(description = "Token description")]
    pub description: Option<String>,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl ProjectToken {
    pub fn new(
        project_id: Partition,
        name: String,
        symbol: String,
        decimals: u8,
        description: Option<String>,
    ) -> Self {
        let now = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: Partition::Token(uuid),
            sk: EntityType::Token,
            project_id,
            gsi1_sk: EntityType::Token,
            name,
            symbol,
            decimals,
            total_supply: 0,
            circulating_supply: 0,
            description,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mint(&mut self, amount: i64) {
        self.total_supply += amount;
        self.circulating_supply += amount;
        self.updated_at = time_utils::get_now();
    }

    pub fn burn(&mut self, amount: i64) -> Result<()> {
        if self.circulating_supply < amount {
            return Err(Error::InsufficientTokens);
        }
        self.total_supply -= amount;
        self.circulating_supply -= amount;
        self.updated_at = time_utils::get_now();
        Ok(())
    }
}
