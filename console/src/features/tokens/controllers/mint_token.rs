use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenBalanceResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[put("/v1/projects/:project_id/tokens/:meta_user_id", auth: ProjectAdminAuth)]
pub async fn mint_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    #[allow(unused_variables)] meta_user_id: String,
    wallet_address: String,
    amount: i64,
) -> Result<TokenBalanceResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let token = ProjectToken::get(cli, &auth.project.pk, Some(EntityType::Token))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let chain_id = token
        .chain_id
        .ok_or(TokenError::MintFailed("Token not deployed".to_string()))?;
    let contract_address = token
        .contract_address
        .as_deref()
        .ok_or(TokenError::MintFailed("Token not deployed".to_string()))?;

    let amount_u256 = ethers::types::U256::from(amount as u64)
        * ethers::types::U256::exp10(18);

    let tx_hash = crate::common::blockchain::evm::transfer_brand_token(
        chain_id,
        contract_address,
        &wallet_address,
        amount_u256,
    )
    .await
    .map_err(|e| TokenError::MintFailed(e))?;

    let now = crate::common::utils::time_utils::get_now();

    Ok(TokenBalanceResponse {
        project_id: auth.project.pk.into(),
        meta_user_id,
        balance: amount,
        tx_hash: Some(format!("{tx_hash:?}")),
        created_at: now,
        updated_at: now,
    })
}
