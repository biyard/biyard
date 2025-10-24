use crate::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct Project {
    #[schemars(description = "ID of the project")]
    pub pk: Partition,
    #[schemars(description = "Entity type of the project")]
    pub sk: EntityType,

    #[schemars(description = "Account ID that owns this project")]
    #[dynamo(index = "gsi1", pk, name = "find_by_account_id")]
    pub account_id: Partition,

    #[schemars(description = "GSI1 sort key (EntityType)")]
    #[dynamo(index = "gsi1", sk, name = "find_by_account_id")]
    pub gsi1_sk: EntityType,

    #[schemars(description = "Name of the project")]
    pub name: String,

    #[schemars(description = "Description of the project")]
    pub description: Option<String>,

    #[schemars(description = "Monthly points supply")]
    pub monthly_points_supply: i64,

    #[schemars(description = "Monthly token supply")]
    pub monthly_token_supply: i64,

    #[schemars(description = "Exchange ratio for point-to-token conversion")]
    pub exchange_ratio: f64,

    #[schemars(description = "Project status")]
    pub status: String,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl Project {
    pub fn new(
        account_id: Partition,
        name: String,
        description: Option<String>,
        monthly_points_supply: i64,
        monthly_token_supply: i64,
        exchange_ratio: f64,
    ) -> Self {
        let now = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: Partition::Project(uuid),
            sk: EntityType::Project,
            account_id,
            gsi1_sk: EntityType::Project,
            name,
            description,
            monthly_points_supply,
            monthly_token_supply,
            exchange_ratio,
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn verify_ownership(&self, account: &crate::features::accounts::Account) -> Result<()> {
        if self.account_id != account.pk {
            return Err(Error::ProjectAccessDenied);
        }
        Ok(())
    }

    pub fn calculate_token_value(&self) -> f64 {
        if self.monthly_token_supply == 0 {
            return 0.0;
        }
        (self.monthly_points_supply as f64) / (self.monthly_token_supply as f64) * self.exchange_ratio
    }

    pub fn calculate_point_to_token_rate(&self) -> f64 {
        if self.monthly_points_supply == 0 {
            return 0.0;
        }
        (self.monthly_token_supply as f64) / (self.monthly_points_supply as f64) * self.exchange_ratio
    }

    pub fn calculate_token_to_point_rate(&self) -> f64 {
        if self.monthly_token_supply == 0 {
            return 0.0;
        }
        (self.monthly_points_supply as f64) / (self.monthly_token_supply as f64) / self.exchange_ratio
    }
}
