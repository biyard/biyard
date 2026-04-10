use crate::common::{ProjectPartition, Result};
use crate::features::tokens::MintDataResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::ProjectAdminAuth;
#[cfg(feature = "server")]
use crate::features::tokens::TokenError;

#[get("/v1/projects/:project_id/tokens/mint-data?to&amount", auth: ProjectAdminAuth)]
pub async fn get_mint_data_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    to: String,
    amount: u64,
) -> Result<MintDataResponse> {
    Err(TokenError::MintFailed(
        "Mint calldata encoding is not supported in the new BrandToken system. Use claim() with server-signed permits instead.".to_string(),
    )
    .into())
}
