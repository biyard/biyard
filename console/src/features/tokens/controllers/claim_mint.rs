use crate::common::{ProjectPartition, Result};
use crate::features::tokens::ClaimResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError, TxClaim};

/// End-user submits their mint tx hash. Server verifies on-chain and records
/// deduplication claim. On-chain contract is the source of truth for balances.
#[post("/v1/projects/:project_id/tokens/claim", auth: ProjectAdminAuth)]
pub async fn claim_mint_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    tx_hash: String,
) -> Result<ClaimResponse> {
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

    // Verify the tx on-chain
    let (to_address, amount) =
        crate::common::blockchain::verify_mint_tx(chain_id, contract_address, &tx_hash)
            .await
            .map_err(|e| TokenError::MintFailed(e))?;

    // Deduplication: try to create TxClaim record (conditional put — fails if already exists)
    let claim = TxClaim::new(
        project.pk.clone(),
        tx_hash.clone(),
        to_address.clone(),
        amount as i64,
        chain_id,
    );
    claim
        .create(cli)
        .await
        .map_err(|_| TokenError::MintFailed("Transaction already claimed".to_string()))?;

    Ok(ClaimResponse {
        tx_hash,
        to: to_address,
        amount,
        chain_id,
    })
}
