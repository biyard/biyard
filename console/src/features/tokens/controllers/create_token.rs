use crate::common::{ProjectPartition, Result};
use crate::features::tokens::{DistributionSlotEntry, TokenResponse};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::projects::Project;
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[post("/v1/projects/:project_id/tokens", auth: ProjectAdminAuth)]
pub async fn create_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    name: String,
    symbol: String,
    description: Option<String>,
    monthly_emission: i64,
    decay_rate_bps: u16,
    distribution_slots: Vec<DistributionSlotEntry>,
    stable_token_address: Option<String>,
    chain_id: Option<u64>,
    start_month: Option<String>,
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
        description,
        monthly_emission,
        decay_rate_bps,
        distribution_slots,
        stable_token_address,
        chain_id,
        start_month,
    );

    let project_update = Project::updater(project.pk.clone(), EntityType::Project)
        .with_monthly_token_supply(monthly_emission)
        .with_updated_at(crate::common::utils::time_utils::get_now());

    crate::transact_write!(
        cli,
        token.create_transact_write_item(),
        project_update.transact_upsert_item(),
    )?;

    Ok(token.into())
}
