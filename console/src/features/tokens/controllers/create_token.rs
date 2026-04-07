use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[post("/v1/projects/:project_id/tokens", auth: ProjectAdminAuth)]
pub async fn create_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    name: String,
    symbol: String,
    decimals: u8,
    description: Option<String>,
    initial_supply: i64,
) -> Result<TokenResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    if ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .is_some()
    {
        return Err(TokenError::TokenAlreadyExists.into());
    }

    let token = ProjectToken::new(
        project.pk.clone(),
        name,
        symbol,
        decimals,
        description,
        initial_supply,
    );

    token.create(cli).await?;

    Ok(token.into())
}
