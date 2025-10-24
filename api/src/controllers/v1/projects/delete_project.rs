use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct DeleteProjectResponse {
    pub success: bool,
}

pub async fn delete_project_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path(project_id): Path<String>,
) -> Result<Json<DeleteProjectResponse>> {
    info!("Deleting project: {}", project_id);

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Delete the project
    Project::delete(&cli, project_pk, Some(EntityType::Project)).await?;

    info!("Project deleted successfully");
    Ok(Json(DeleteProjectResponse { success: true }))
}
