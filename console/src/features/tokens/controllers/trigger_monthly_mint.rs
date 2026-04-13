use crate::common::{ProjectPartition, Result};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerMintResponse {
    pub tx_hash: String,
    pub month: String,
}

#[post("/v1/projects/:project_id/tokens/trigger-monthly-mint", auth: ProjectAdminAuth)]
pub async fn trigger_monthly_mint_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<TriggerMintResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (token_pk, token_sk) = ProjectToken::keys(project_id.clone().into());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let multisig_addr = token
        .multisig_address
        .as_deref()
        .ok_or_else(|| TokenError::DeployFailed("Multisig not deployed".to_string()))?;
    let token_addr = token
        .contract_address
        .as_deref()
        .ok_or_else(|| TokenError::DeployFailed("Token not deployed".to_string()))?;
    let chain_id = token
        .chain_id
        .ok_or_else(|| TokenError::DeployFailed("Chain ID not set".to_string()))?;

    let current_month = crate::common::utils::time_utils::timestamp_to_yyyy_mm();

    if token.last_minted_month.as_deref() == Some(current_month.as_str()) {
        return Err(
            TokenError::MintFailed(format!("Already minted this month ({current_month})")).into(),
        );
    }

    let tx_hash =
        crate::common::blockchain::trigger_monthly_mint(chain_id, multisig_addr, token_addr)
            .await
            .map_err(TokenError::DeployFailed)?;

    let now = crate::common::utils::time_utils::get_now();
    ProjectToken::updater(token.pk, token.sk)
        .with_last_minted_month(current_month.clone())
        .with_updated_at(now)
        .execute(cli)
        .await?;

    Ok(TriggerMintResponse {
        tx_hash: format!("{tx_hash:?}"),
        month: current_month,
    })
}
