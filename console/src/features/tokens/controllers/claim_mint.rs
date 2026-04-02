use crate::common::{ProjectPartition, Result};
use crate::features::tokens::ClaimResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenBalance, TokenError, TxClaim};

/// End-user submits their mint tx hash. Server verifies on-chain, then updates DB.
/// Deduplication: TxClaim record prevents the same tx_hash from being claimed twice.
#[post("/v1/projects/:project_id/tokens/claim", auth: ProjectAuth)]
pub async fn claim_mint_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    tx_hash: String,
) -> Result<ClaimResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let mut token = ProjectToken::get(cli, &token_pk, Some(token_sk))
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

    // Update off-chain token supply
    token.mint(amount as i64);

    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_total_supply(token.total_supply)
        .with_circulating_supply(token.circulating_supply)
        .with_updated_at(token.updated_at)
        .execute(cli)
        .await?;

    // Update user balance
    let meta_user_id = to_address.clone();
    let (balance_pk, balance_sk) =
        TokenBalance::keys(project.pk.clone(), meta_user_id.clone());
    let mut balance = TokenBalance::get(cli, &balance_pk, Some(balance_sk))
        .await?
        .unwrap_or_else(|| TokenBalance::new(project.pk.clone(), meta_user_id));

    balance.add_tokens(amount as i64);

    if balance.created_at == balance.updated_at {
        balance.create(cli).await?;
    } else {
        TokenBalance::updater(balance.pk.clone(), balance.sk.clone())
            .with_balance(balance.balance)
            .with_updated_at(balance.updated_at)
            .execute(cli)
            .await?;
    }

    Ok(ClaimResponse {
        tx_hash,
        to: to_address,
        amount,
        chain_id,
    })
}
