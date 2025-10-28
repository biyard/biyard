use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;
use validator::Validate;

pub async fn create_project_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>> {
    debug!("Creating project for account: {:?}", account.pk);

    // Validate the request
    req.validate()?;

    // Create the project
    let project = Project::new(
        account.pk.clone(),
        req.name,
        req.description,
        req.monthly_token_supply,
    );

    // Save to DynamoDB
    project.create(&cli).await?;

    debug!("Project created: {:?}", project.pk);
    Ok(Json(project.into()))
}
