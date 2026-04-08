use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};
#[cfg(feature = "server")]
use ethers::types::Address;

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

    if token.treasury_contract_address.is_some() {
        return Err(TokenError::AlreadyDeployed.into());
    }

    if let Some(existing_chain_id) = token.chain_id {
        if token.contract_address.is_some() && existing_chain_id != chain_id {
            return Err(TokenError::DeployFailed(format!(
                "Token is already deployed on chain {existing_chain_id}; treasury must be deployed on the same chain"
            ))
            .into());
        }
    }

    let treasury_reserve_bps =
        (project.treasury_reserve_rate.clamp(0.0, 1.0) * 10000.0).round() as u64;
    let now = crate::common::utils::time_utils::get_now();
    let mut updater =
        ProjectToken::updater(token.pk.clone(), token.sk.clone()).with_updated_at(now);

    let (contract_address, token_addr_str, deployment_tx_hash) =
        if let Some(existing_token_address) = token.contract_address.as_deref() {
            let parsed = existing_token_address.parse::<Address>().map_err(|e| {
                TokenError::DeployFailed(format!("Invalid existing token contract address: {e}"))
            })?;

            updater = updater.with_chain_id(token.chain_id.unwrap_or(chain_id));
            (
                parsed,
                existing_token_address.to_string(),
                token.deployment_tx_hash.clone(),
            )
        } else {
            let (contract_address, tx_hash) = crate::common::blockchain::deploy_token(
                chain_id,
                &token.name,
                &token.symbol,
                token.total_supply.max(0) as u64,
                0,
            )
            .await
            .map_err(TokenError::DeployFailed)?;

            let token_addr_str = format!("{contract_address:?}");
            let tx_str = format!("{tx_hash:?}");
            updater = updater
                .with_contract_address(token_addr_str.clone())
                .with_chain_id(chain_id)
                .with_deployment_tx_hash(tx_str.clone());

            (contract_address, token_addr_str, Some(tx_str))
        };

    let (treasury_contract_address, treasury_tx_hash, stable_token_address, _project_owner) =
        crate::common::blockchain::deploy_floor_price_treasury(
            chain_id,
            contract_address,
            treasury_reserve_bps,
        )
        .await
        .map_err(|e| TokenError::DeployFailed(e))?;

    let treasury_addr_str = format!("{treasury_contract_address:?}");
    let stable_addr_str = format!("{stable_token_address:?}");

    crate::common::blockchain::add_minter(chain_id, &token_addr_str, &treasury_addr_str)
        .await
        .map_err(|e| TokenError::DeployFailed(e))?;

    let treasury_tx_str = format!("{treasury_tx_hash:?}");

    updater
        .with_treasury_contract_address(treasury_addr_str.clone())
        .with_stable_token_address(stable_addr_str.clone())
        .with_chain_id(chain_id)
        .with_treasury_deployment_tx_hash(treasury_tx_str.clone())
        .with_treasury_reserve_bps(treasury_reserve_bps)
        .execute(cli)
        .await?;

    let mut updated = token;
    updated.contract_address = Some(token_addr_str);
    updated.treasury_contract_address = Some(treasury_addr_str);
    updated.stable_token_address = Some(stable_addr_str);
    updated.chain_id = Some(chain_id);
    if deployment_tx_hash.is_some() {
        updated.deployment_tx_hash = deployment_tx_hash;
    }
    updated.treasury_deployment_tx_hash = Some(treasury_tx_str);
    updated.treasury_reserve_bps = treasury_reserve_bps;
    updated.updated_at = now;

    Ok(updated.into())
}
