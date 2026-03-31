use crate::common::{EntityType, ProjectPartition, Result, Serialize};
use dioxus::prelude::delete;
use serde::Deserialize;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::projects::Project;

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
