use crate::features::projects::*;
use crate::features::tokens::*;
use crate::*;

pub async fn get_token_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    // Path(ProjectPathParam { project_id }): ProjectPath,
) -> Result<Json<TokenResponse>> {
    info!("Getting token for project: {}", project.pk);

    let token = ProjectToken::get(&cli, &project.pk, Some(EntityType::Token))
        .await?
        .ok_or(Error::TokenNotFound)?;

    Ok(Json(token.into()))
}

pub async fn get_token_balance_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(ProjectUserPathParam { meta_user_id, .. }): ProjectUserPath,
) -> Result<Json<TokenBalanceResponse>> {
    info!(
        "Getting token balance for user {} in project: {:?}",
        meta_user_id, project.pk
    );

    let (pk, sk) = TokenBalance::keys(project.pk.into(), meta_user_id.clone());

    let balance = TokenBalance::get(&cli, &pk, Some(sk))
        .await?
        .ok_or(Error::TokenBalanceNotFound)?;

    Ok(Json(balance.into()))
}
