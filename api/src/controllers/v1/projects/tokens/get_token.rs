use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::features::tokens::*;
use crate::*;

pub async fn get_token_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path((project_id, token_id)): Path<(String, String)>,
) -> Result<Json<TokenResponse>> {
    info!("Getting token: {} in project: {}", token_id, project_id);

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Get the token
    let token_pk = Partition::Token(token_id);
    let token = ProjectToken::get(&cli, token_pk, Some(EntityType::Token))
        .await?
        .ok_or(Error::TokenNotFound)?;

    // Verify token belongs to this project
    if token.project_id != project_pk {
        return Err(Error::TokenNotFound);
    }

    Ok(Json(token.into()))
}

pub async fn get_token_balance_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path((project_id, token_id, meta_user_id)): Path<(String, String, String)>,
) -> Result<Json<TokenBalanceResponse>> {
    info!("Getting token balance for user {} in token: {} project: {}", meta_user_id, token_id, project_id);

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Get the token to verify it exists
    let token_pk = Partition::Token(token_id);
    let token = ProjectToken::get(&cli, token_pk.clone(), Some(EntityType::Token))
        .await?
        .ok_or(Error::TokenNotFound)?;

    // Verify token belongs to this project
    if token.project_id != project_pk {
        return Err(Error::TokenNotFound);
    }

    // Get token balance
    let balance_pk = format!("{}#USER#{}", token_pk.to_string(), meta_user_id);
    let balance_pk = Partition::TokenBalance(balance_pk);

    let balance = TokenBalance::get(&cli, balance_pk, Some(EntityType::TokenBalance))
        .await?
        .unwrap_or_else(|| TokenBalance::new(token_pk, project_pk, meta_user_id));

    Ok(Json(balance.into()))
}
