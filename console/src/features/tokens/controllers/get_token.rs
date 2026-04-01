use crate::common::{ProjectPartition, Result};
use crate::features::tokens::{TokenBalanceResponse, TokenResponse};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenBalance, TokenError};

#[get("/v1/projects/:project_id/tokens", auth: ProjectAuth)]
pub async fn get_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<TokenResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let token = ProjectToken::get(cli, &auth.project.pk, Some(EntityType::Token))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    Ok(token.into())
}

#[get("/v1/projects/:project_id/tokens/balance/:meta_user_id", auth: ProjectAuth)]
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
