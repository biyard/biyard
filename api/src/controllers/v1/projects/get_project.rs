use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;

pub async fn get_project_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectResponse>> {
    info!("Getting project: {}", project_id);

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk, Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    Ok(Json(project.into()))
}
