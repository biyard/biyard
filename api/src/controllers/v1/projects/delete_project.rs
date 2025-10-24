use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct DeleteProjectResponse {
    pub success: bool,
}

pub async fn delete_project_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(Project { pk: project_pk, .. }): Extension<Project>,
    Path(_p): ProjectPath,
) -> Result<Json<DeleteProjectResponse>> {
    debug!("Deleting project: {}", project_pk);

    // Delete the project
    Project::delete(&cli, project_pk, Some(EntityType::Project)).await?;

    Ok(Json(DeleteProjectResponse { success: true }))
}
