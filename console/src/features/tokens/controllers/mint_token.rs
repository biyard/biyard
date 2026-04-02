use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenBalanceResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[put("/v1/projects/:project_id/tokens/:meta_user_id", auth: ProjectAuth)]
pub async fn mint_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    amount: i64,
    #[allow(unused_variables)] description: Option<String>,
) -> Result<TokenBalanceResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let (contract_address, chain_id) = match (&token.contract_address, token.chain_id) {
        (Some(addr), Some(chain)) => (addr.clone(), chain),
        _ => return Err(TokenError::NotDeployed.into()),
    };

    let to_address = if meta_user_id.starts_with("0x") && meta_user_id.len() == 42 {
        meta_user_id.clone()
    } else {
        let wallet: ethers::signers::LocalWallet = std::env::var("DEPLOYER_PRIVATE_KEY")
            .unwrap_or_default()
            .parse()
            .map_err(|_| TokenError::MintFailed("Invalid deployer key".to_string()))?;
        format!("{:?}", ethers::utils::secret_key_to_address(&wallet.signer()))
    };

    let hash = crate::common::blockchain::mint_on_chain(
        chain_id,
        &contract_address,
        &to_address,
        amount.max(0) as u64,
    )
    .await
    .map_err(|e| TokenError::MintFailed(e))?;

    let now = chrono::Utc::now().timestamp();
    Ok(TokenBalanceResponse {
        project_id: project.pk.clone(),
        meta_user_id,
        balance: amount,
        tx_hash: Some(format!("{hash:?}")),
        created_at: now,
        updated_at: now,
    })
}
