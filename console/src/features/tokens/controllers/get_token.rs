use crate::common::{ProjectPartition, Result};
use crate::features::tokens::{TokenBalanceResponse, TokenResponse};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenBalance, TokenError};

/// Returns the token configured for this project, or `None` if the brand
/// has not yet defined a token. Returning `Ok(None)` instead of a 404
/// keeps "no token yet" out of the browser console error log and lets the
/// frontend distinguish "loading", "empty", and "error" states cleanly.
#[api_doc_macros::api_doc(group = "Tokens", summary = "Get token")]
#[get("/v1/projects/:project_id/tokens", auth: ProjectViewerAuth)]
pub async fn get_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<Option<TokenResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let token = ProjectToken::get(cli, &auth.project.pk, Some(EntityType::Token)).await?;

    Ok(token.map(Into::into))
}

#[get("/v1/projects/:project_id/tokens/balance/:meta_user_id", auth: ProjectViewerAuth)]
pub async fn get_token_balance_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
) -> Result<TokenBalanceResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (pk, sk) = TokenBalance::keys(auth.project.pk, meta_user_id);
    let balance = TokenBalance::get(cli, &pk, Some(sk))
        .await?
        .ok_or(TokenError::TokenBalanceNotFound)?;

    Ok(balance.into())
}
