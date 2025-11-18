use crate::{features::projects::ProjectStatus, *};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ProjectResponse {
    #[schemars(description = "ID of the project")]
    pub id: String,

    #[schemars(description = "Account ID that owns this project")]
    pub account_id: Partition,

    #[schemars(description = "Name of the project")]
    pub name: String,

    #[schemars(description = "Description of the project")]
    pub description: Option<String>,

    #[schemars(description = "Monthly token supply")]
    pub monthly_token_supply: i64,

    #[schemars(description = "Project status")]
    pub status: ProjectStatus,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl From<crate::features::projects::Project> for ProjectResponse {
    fn from(project: crate::features::projects::Project) -> Self {
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
