use crate::common::{ProjectPartition, Result};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};
#[cfg(feature = "server")]
use ethers::prelude::*;
#[cfg(feature = "server")]
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionSlotInput {
    pub wallet: String,
    pub bps: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDistributionSlotsResponse {
    pub tx_hash: String,
}

#[post("/v1/projects/:project_id/tokens/distribution-slots", auth: ProjectAdminAuth)]
pub async fn set_distribution_slots_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    slots: Vec<DistributionSlotInput>,
) -> Result<SetDistributionSlotsResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (token_pk, token_sk) = ProjectToken::keys(project_id.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let multisig_addr_str = token
        .multisig_address
        .as_deref()
        .ok_or_else(|| TokenError::DeployFailed("Multisig not deployed".to_string()))?;
    let token_addr_str = token
        .contract_address
        .as_deref()
        .ok_or_else(|| TokenError::DeployFailed("Token not deployed".to_string()))?;
    let chain_id = token
        .chain_id
        .ok_or_else(|| TokenError::DeployFailed("Chain ID not set".to_string()))?;

    let client = Arc::new(crate::common::blockchain::signer(chain_id)?);
    let ms_addr: Address = multisig_addr_str
        .parse()
        .map_err(|e| TokenError::DeployFailed(format!("Invalid multisig address: {e}")))?;
    let token_addr: Address = token_addr_str
        .parse()
        .map_err(|e| TokenError::DeployFailed(format!("Invalid token address: {e}")))?;

    let wallets: Vec<Address> = slots
        .iter()
        .map(|s| {
            s.wallet
                .parse::<Address>()
                .map_err(|e| TokenError::DeployFailed(format!("Invalid wallet address: {e}")))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let bps: Vec<u16> = slots.iter().map(|s| s.bps).collect();

    let token_contract =
        crate::common::blockchain::BrandTokenContract::new(token_addr, client.clone());
    let calldata = token_contract
        .set_distribution_slots(wallets, bps)
        .calldata()
        .ok_or_else(|| {
            TokenError::DeployFailed("Failed to encode setDistributionSlots calldata".to_string())
        })?;

    let ms = crate::common::blockchain::MultisigContract::new(ms_addr, client.clone());

    let proposal_count = ms
        .proposal_count()
        .call()
        .await
        .map_err(|e| TokenError::DeployFailed(format!("proposalCount failed: {e}")))?;

    ms.propose(token_addr, calldata.to_vec().into(), U256::zero())
        .send()
        .await
        .map_err(|e| TokenError::DeployFailed(format!("propose failed: {e}")))?
        .await
        .map_err(|e| TokenError::DeployFailed(format!("propose receipt failed: {e}")))?;

    ms.approve(proposal_count)
        .send()
        .await
        .map_err(|e| TokenError::DeployFailed(format!("approve failed: {e}")))?
        .await
        .map_err(|e| TokenError::DeployFailed(format!("approve receipt failed: {e}")))?;

    let pending = ms
        .execute(proposal_count)
        .send()
        .await
        .map_err(|e| TokenError::DeployFailed(format!("execute failed: {e}")))?;

    let tx_hash = pending.tx_hash();
    pending
        .await
        .map_err(|e| TokenError::DeployFailed(format!("execute receipt failed: {e}")))?;

    Ok(SetDistributionSlotsResponse {
        tx_hash: format!("{tx_hash:?}"),
    })
}
