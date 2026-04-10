use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[cfg(feature = "server")]
fn compute_max_supply(monthly_emission: u64, decay_bps: u16, months: u32) -> u64 {
    let mut total: u128 = 0;
    let mut emission = monthly_emission as u128;
    for _ in 0..months {
        total += emission;
        emission = emission * (10000 - decay_bps as u128) / 10000;
    }
    total as u64
}

#[post("/v1/projects/:project_id/tokens/deploy", auth: ProjectAdminAuth)]
pub async fn deploy_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    chain_id: u64,
) -> Result<TokenResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    crate::common::SupportedChain::from_chain_id(chain_id)
        .ok_or_else(|| TokenError::DeployFailed(format!("Unsupported chain: {chain_id}")))?;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    if token.contract_address.is_some() {
        return Err(TokenError::AlreadyDeployed.into());
    }

    let monthly_emission = project.monthly_token_supply.max(0) as u64;
    let decay_rate_bps: u16 = 500;
    let max_supply = compute_max_supply(monthly_emission, decay_rate_bps, 60);

    let deployment = crate::common::blockchain::deploy_brand_system(
        chain_id,
        &token.name,
        &token.symbol,
        max_supply,
        monthly_emission,
        decay_rate_bps,
    )
    .await
    .map_err(TokenError::DeployFailed)?;

    let token_addr_str = format!("{:?}", deployment.token_address);
    let treasury_addr_str = format!("{:?}", deployment.treasury_address);
    let multisig_addr_str = format!("{:?}", deployment.multisig_address);
    let stable_addr_str = format!("{:?}", deployment.stable_token_address);
    let token_tx_str = format!("{:?}", deployment.token_tx_hash);
    let treasury_tx_str = format!("{:?}", deployment.treasury_tx_hash);
    let multisig_tx_str = format!("{:?}", deployment.multisig_tx_hash);

    let now = crate::common::utils::time_utils::get_now();
    let treasury_reserve_bps =
        (project.treasury_reserve_rate.clamp(0.0, 1.0) * 10000.0).round() as u64;

    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_contract_address(token_addr_str.clone())
        .with_treasury_contract_address(treasury_addr_str.clone())
        .with_multisig_address(multisig_addr_str.clone())
        .with_stable_token_address(stable_addr_str.clone())
        .with_chain_id(chain_id)
        .with_deployment_tx_hash(token_tx_str.clone())
        .with_treasury_deployment_tx_hash(treasury_tx_str.clone())
        .with_multisig_deployment_tx_hash(multisig_tx_str.clone())
        .with_treasury_reserve_bps(treasury_reserve_bps)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    let mut updated = token;
    updated.contract_address = Some(token_addr_str);
    updated.treasury_contract_address = Some(treasury_addr_str);
    updated.multisig_address = Some(multisig_addr_str);
    updated.stable_token_address = Some(stable_addr_str);
    updated.chain_id = Some(chain_id);
    updated.deployment_tx_hash = Some(token_tx_str);
    updated.treasury_deployment_tx_hash = Some(treasury_tx_str);
    updated.multisig_deployment_tx_hash = Some(multisig_tx_str);
    updated.treasury_reserve_bps = treasury_reserve_bps;
    updated.updated_at = now;

    Ok(updated.into())
}
