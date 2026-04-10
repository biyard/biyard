use crate::common::{ProjectPartition, Result};
use crate::features::tokens::AddMinterResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::ProjectAdminAuth;
#[cfg(feature = "server")]
use crate::features::tokens::TokenError;

#[post("/v1/projects/:project_id/tokens/minters", auth: ProjectAdminAuth)]
pub async fn add_minter_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    #[allow(unused_variables)] wallet_address: String,
) -> Result<AddMinterResponse> {
    Err(TokenError::MintFailed(
        "Minter role is not supported in the new BrandToken system".to_string(),
    )
    .into())
}
