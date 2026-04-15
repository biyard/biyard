use crate::common::{ProjectPartition, Result};
use crate::features::projects::TreasuryStatusResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectViewerAuth, SupportedChain};
#[cfg(feature = "server")]
use crate::features::tokens::ProjectToken;

#[api_doc_macros::api_doc(group = "Projects", summary = "Get treasury status")]
#[get("/v1/projects/:project_id/treasury/status", auth: ProjectViewerAuth)]
pub async fn get_treasury_status_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<TreasuryStatusResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    // The treasury contract address lives on the project's token
    // record; the project itself no longer caches on-chain state.
    let token = ProjectToken::get(cli, &auth.project.pk, Some(EntityType::Token)).await?;

    let Some(token) = token else {
        return Ok(TreasuryStatusResponse::default());
    };

    let (Some(treasury_address), Some(brand_token_address), Some(chain_id)) = (
        token.treasury_contract_address.as_deref(),
        token.contract_address.as_deref(),
        token.chain_id,
    ) else {
        return Ok(TreasuryStatusResponse::default());
    };

    // Fetch the live snapshot from chain. If the RPC call fails we
    // surface it as an error so the UI can show "unavailable" rather
    // than silently returning zeros that look like a deployed-but-
    // empty treasury.
    let status = crate::common::blockchain::get_treasury_status(
        chain_id,
        treasury_address,
        brand_token_address,
    )
    .await
    .map_err(crate::common::Error::InternalServerError)?;

    let stable_mintable = token
        .stable_token_address
        .as_deref()
        .and_then(|addr| {
            SupportedChain::from_chain_id(chain_id).and_then(|chain| {
                chain
                    .stable_token_options()
                    .into_iter()
                    .find(|opt| opt.address.eq_ignore_ascii_case(addr))
                    .map(|opt| opt.mintable)
            })
        })
        .unwrap_or(false);

    Ok(TreasuryStatusResponse {
        deployed: true,
        chain_id: Some(chain_id),
        treasury_contract_address: Some(treasury_address.to_string()),
        brand_token_address: Some(brand_token_address.to_string()),
        treasury_balance_raw: status.treasury_balance_raw.to_string(),
        stable_decimals: status.stable_decimals,
        stable_symbol: status.stable_symbol,
        stable_mintable,
        total_supply_raw: status.total_supply_raw.to_string(),
        circulating_supply_raw: status.circulating_supply_raw.to_string(),
        treasury_held_tokens_raw: status.treasury_held_tokens_raw.to_string(),
        token_decimals: status.token_decimals,
        token_symbol: status.token_symbol,
        floor_price_raw_1e18: status.floor_price_raw_1e18.to_string(),
        current_month: status.current_month,
    })
}
