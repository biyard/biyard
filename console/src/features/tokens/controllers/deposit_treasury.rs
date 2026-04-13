use crate::common::{ProjectPartition, Result};
use crate::features::tokens::DepositResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[post("/v1/projects/:project_id/tokens/treasury/deposit", auth: ProjectAdminAuth)]
pub async fn deposit_treasury_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    amount: i64,
) -> Result<DepositResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let token = ProjectToken::get(cli, &auth.project.pk, Some(EntityType::Token))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let chain_id = token
        .chain_id
        .ok_or(TokenError::DepositFailed("Token not deployed".to_string()))?;
    let stable_address = token
        .stable_token_address
        .as_deref()
        .ok_or(TokenError::DepositFailed(
            "Stable token address not set".to_string(),
        ))?;
    let treasury_address = token
        .treasury_contract_address
        .as_deref()
        .ok_or(TokenError::DepositFailed(
            "Treasury not deployed".to_string(),
        ))?;

    let amount_u256 = ethers::types::U256::from(amount as u64)
        * ethers::types::U256::exp10(6); // BUSDT has 6 decimals

    let tx_hash = crate::common::blockchain::evm::deposit_stable_to_treasury(
        chain_id,
        stable_address,
        treasury_address,
        amount_u256,
    )
    .await
    .map_err(|e| TokenError::DepositFailed(e))?;

    Ok(DepositResponse {
        tx_hash: format!("{tx_hash:?}"),
        amount: amount.to_string(),
    })
}
