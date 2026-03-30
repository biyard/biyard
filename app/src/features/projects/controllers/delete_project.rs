use crate::common::{CommonConfig, EntityType, ProjectAuth, ProjectPartition, Result, Serialize};
use crate::features::projects::Project;
use dioxus::prelude::delete;
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProjectResponse {
    pub success: bool,
}

#[delete("/v1/projects/:project_id", auth: ProjectAuth)]
pub async fn delete_project_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<DeleteProjectResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    Project::delete(cli, auth.project.pk, Some(EntityType::Project)).await?;

    Ok(DeleteProjectResponse { success: true })
}
