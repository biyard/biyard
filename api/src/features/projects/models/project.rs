use crate::{features::projects::ProjectStatus, *};

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

    #[schemars(description = "Monthly automatic token supply")]
    #[serde(default)]
    pub monthly_token_supply: i64,

    #[schemars(description = "Project status")]
    pub status: ProjectStatus,

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
        monthly_token_supply: i64,
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
            monthly_token_supply,
            status: ProjectStatus::Active,
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
}
