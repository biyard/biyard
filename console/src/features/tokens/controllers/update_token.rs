use crate::common::{ProjectPartition, Result};
use crate::features::tokens::{DistributionSlotEntry, TokenResponse};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[api_doc_macros::api_doc(group = "Tokens", summary = "Update token")]
#[put("/v1/projects/:project_id/tokens", auth: ProjectAdminAuth)]
pub async fn update_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    name: Option<String>,
    symbol: Option<String>,
    description: Option<String>,
    monthly_emission: Option<i64>,
    decay_rate_bps: Option<u16>,
    distribution_slots: Option<Vec<DistributionSlotEntry>>,
    stable_token_address: Option<String>,
    chain_id: Option<u64>,
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
    if let Some(description) = description {
        updater = updater.with_description(description);
    }
    if let Some(monthly_emission) = monthly_emission {
        updater = updater.with_monthly_emission(monthly_emission);
    }
    if let Some(decay_rate_bps) = decay_rate_bps {
        updater = updater.with_decay_rate_bps(decay_rate_bps);
    }
    if let Some(distribution_slots) = distribution_slots {
        updater = updater.with_distribution_slots(distribution_slots);
    }
    if let Some(stable_token_address) = stable_token_address {
        updater = updater.with_stable_token_address(stable_token_address);
    }
    if let Some(chain_id) = chain_id {
        updater = updater.with_chain_id(chain_id);
    }

    updater = updater.with_updated_at(now);
    let updated = updater.execute(cli).await?;

    Ok(updated.into())
}
