use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[put("/v1/projects/:project_id/tokens", auth: ProjectAdminAuth)]
pub async fn update_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    name: Option<String>,
    symbol: Option<String>,
    decimals: Option<u8>,
    description: Option<String>,
) -> Result<TokenResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let token = ProjectToken::get(cli, &project.pk, Some(EntityType::Token))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    if token.contract_address.is_some() {
        return Err(TokenError::AlreadyDeployed.into());
    }

    let now = crate::common::utils::time_utils::get_now();
    let mut updater = ProjectToken::updater(token.pk.clone(), token.sk.clone());

    if let Some(name) = name {
        updater = updater.with_name(name);
    }
    if let Some(symbol) = symbol {
        updater = updater.with_symbol(symbol);
    }
    if let Some(decimals) = decimals {
        updater = updater.with_decimals(decimals);
    }
    if let Some(description) = description {
        updater = updater.with_description(description);
    }

    updater = updater.with_updated_at(now);
    let updated = updater.execute(cli).await?;

    Ok(updated.into())
}
