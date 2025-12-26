use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::features::tokens::*;
use crate::*;
use validator::Validate;

pub async fn create_token_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path(project_id): Path<String>,
    Json(req): Json<CreateTokenRequest>,
) -> Result<Json<TokenResponse>> {
    info!("Creating token for project: {}", project_id);

    // Validate the request
    req.validate()?;

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Check if a token with this symbol already exists for this project
    let (existing_tokens, _) =
        ProjectToken::find_by_project(&cli, &project_pk, ProjectTokenQueryOption::builder())
            .await?;

    if existing_tokens.iter().any(|t| t.symbol == req.symbol) {
        return Err(Error::TokenAlreadyExists);
    }

    // Create the token
    let token = ProjectToken::new(
        project_pk,
        req.name,
        req.symbol,
        req.decimals,
        req.description,
    );

    // Save to DynamoDB
    token.create(&cli).await?;

    info!("Token created: {:?}", token.pk);
    Ok(Json(token.into()))
}
