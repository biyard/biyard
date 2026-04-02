use crate::common::{ProjectPartition, Result};
use crate::features::tokens::MintDataResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

/// Returns encoded calldata for the user to sign a mint tx in their wallet.
/// No gas cost — just returns data.
#[get("/v1/projects/:project_id/tokens/mint-data?to&amount", auth: ProjectAuth)]
pub async fn get_mint_data_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    to: String,
    amount: u64,
) -> Result<MintDataResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let contract_address = token
        .contract_address
        .clone()
        .ok_or(TokenError::NotDeployed)?;

    let chain_id = token.chain_id.ok_or(TokenError::NotDeployed)?;

    let calldata = crate::common::blockchain::encode_mint_calldata(&to, amount)
        .map_err(|e| TokenError::MintFailed(e))?;

    Ok(MintDataResponse {
        contract_address,
        chain_id,
        calldata,
        to,
        amount,
    })
}
