use crate::common::{Deserialize, DynamoEntity, EntityType, Partition, Result, Serialize};
use crate::features::projects::{ProjectError, ProjectStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Project {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", pk, name = "find_by_account_id")]
    pub account_id: Partition,

    #[dynamo(index = "gsi1", sk, name = "find_by_account_id")]
    pub gsi1_sk: EntityType,

    pub name: String,
    pub description: Option<String>,

    #[serde(default)]
    pub monthly_token_supply: i64,

    pub status: ProjectStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Project {
    pub fn new(
        account_id: Partition,
        name: String,
        description: Option<String>,
        monthly_token_supply: i64,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();
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

    pub fn verify_ownership(&self, account_pk: &Partition) -> Result<()> {
        if self.account_id != *account_pk {
            return Err(ProjectError::ProjectAccessDenied.into());
        }
        Ok(())
    }
}

impl From<Project> for crate::features::projects::ProjectResponse {
    fn from(project: Project) -> Self {
        let project_id = match &project.pk {
            Partition::Project(id) => id.clone(),
            _ => "".to_string(),
        };

        Self {
            id: project_id,
            account_id: project.account_id,
            name: project.name,
            description: project.description,
            monthly_token_supply: project.monthly_token_supply,
            status: project.status,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
