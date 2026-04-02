use crate::common::{ProjectPartition, Result};
use crate::features::tokens::AddMinterResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

/// Register a wallet address as a minter on the deployed contract.
/// Server pays gas. Called by the project admin from the console.
#[post("/v1/projects/:project_id/tokens/minters", auth: ProjectAuth)]
pub async fn add_minter_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    wallet_address: String,
) -> Result<AddMinterResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let contract_address = token
        .contract_address
        .as_deref()
        .ok_or(TokenError::NotDeployed)?;

    let chain_id = token.chain_id.ok_or(TokenError::NotDeployed)?;

    // Check if already a minter
    let already = crate::common::blockchain::is_minter(chain_id, contract_address, &wallet_address)
        .await
        .unwrap_or(false);

    if already {
        return Ok(AddMinterResponse {
            minter_address: wallet_address,
            tx_hash: String::new(),
            chain_id,
        });
    }

    let tx_hash =
        crate::common::blockchain::add_minter(chain_id, contract_address, &wallet_address)
            .await
            .map_err(|e| TokenError::MintFailed(e))?;

    Ok(AddMinterResponse {
        minter_address: wallet_address,
        tx_hash: format!("{tx_hash:?}"),
        chain_id,
    })
}
