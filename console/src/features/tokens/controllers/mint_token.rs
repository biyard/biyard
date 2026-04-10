use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenBalanceResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::ProjectAdminAuth;
#[cfg(feature = "server")]
use crate::features::tokens::TokenError;

#[put("/v1/projects/:project_id/tokens/:meta_user_id", auth: ProjectAdminAuth)]
pub async fn mint_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    amount: i64,
    #[allow(unused_variables)] description: Option<String>,
) -> Result<TokenBalanceResponse> {
    Err(TokenError::MintFailed(
        "Direct minting is not supported in the new BrandToken system. Use triggerMonthlyMint via multisig instead.".to_string(),
    )
    .into())
}
