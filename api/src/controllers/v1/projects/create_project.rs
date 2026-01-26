use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::features::tokens::ProjectToken;
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
        req.name.clone(),
        req.description.clone(),
        req.monthly_token_supply,
    );

    // Save to DynamoDB
    let token = ProjectToken::new(
        project.pk.clone(),
        req.name,
        req.symbol,
        req.decimals,
        req.description,
    );
    transact_write_items!(
        &cli,
        vec![
            project.create_transact_write_item(),
            token.create_transact_write_item()
        ]
    )?;

    Ok(Json(project.into()))
}
