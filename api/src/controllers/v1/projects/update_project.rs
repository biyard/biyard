use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;
use validator::Validate;

pub async fn update_project_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path(project_id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>> {
    info!("Updating project: {}", project_id);

    // Validate the request
    req.validate()?;

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Build updater
    let mut updater = Project::updater(project_pk, EntityType::Project);

    if let Some(name) = req.name {
        updater = updater.with_name(name);
    }
    if let Some(description) = req.description {
        updater = updater.with_description(description);
    }
    if let Some(monthly_points_supply) = req.monthly_points_supply {
        updater = updater.with_monthly_points_supply(monthly_points_supply);
    }
    if let Some(monthly_token_supply) = req.monthly_token_supply {
        updater = updater.with_monthly_token_supply(monthly_token_supply);
    }
    if let Some(exchange_ratio) = req.exchange_ratio {
        updater = updater.with_exchange_ratio(exchange_ratio);
    }
    if let Some(status) = req.status {
        updater = updater.with_status(status);
    }

    // Always update the updated_at timestamp
    updater = updater.with_updated_at(time_utils::get_now());

    // Execute update
    let updated_project = updater.execute(&cli).await?;

    Ok(Json(updated_project.into()))
}
