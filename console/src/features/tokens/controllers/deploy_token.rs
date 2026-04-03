use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

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

    let (contract_address, tx_hash) = crate::common::blockchain::deploy_token(
        chain_id,
        &token.name,
        &token.symbol,
        token.total_supply.max(0) as u64,
        0,
    )
    .await
    .map_err(|e| TokenError::DeployFailed(e))?;

    let now = crate::common::utils::time_utils::get_now();
    let addr_str = format!("{contract_address:?}");
    let tx_str = format!("{tx_hash:?}");

    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_contract_address(addr_str.clone())
        .with_chain_id(chain_id)
        .with_deployment_tx_hash(tx_str.clone())
        .with_updated_at(now)
        .execute(cli)
        .await?;

    let mut updated = token;
    updated.contract_address = Some(addr_str);
    updated.chain_id = Some(chain_id);
    updated.deployment_tx_hash = Some(tx_str);
    updated.updated_at = now;

    Ok(updated.into())
}
