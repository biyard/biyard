use crate::common::{ProjectPartition, Result};
use crate::features::tokens::TokenResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};
#[cfg(feature = "server")]
use ethers::prelude::*;

#[post("/v1/projects/:project_id/tokens/deploy", auth: ProjectAdminAuth)]
pub async fn deploy_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<TokenResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    if token.contract_address.is_some() {
        return Err(TokenError::AlreadyDeployed.into());
    }

    if token.deploying {
        return Err(TokenError::DeployFailed("Deployment already in progress".to_string()).into());
    }

    let chain_id = token
        .chain_id
        .ok_or_else(|| TokenError::DeployFailed("Chain not configured".to_string()))?;
    crate::common::SupportedChain::from_chain_id(chain_id)
        .ok_or_else(|| TokenError::DeployFailed(format!("Unsupported chain: {chain_id}")))?;

    let monthly_emission_raw = token.monthly_emission.max(0) as u64;
    if monthly_emission_raw == 0 {
        return Err(TokenError::DeployFailed("Monthly emission not configured".to_string()).into());
    }
    // Scale by 10^18 so the on-chain value matches ERC-20 units.
    // BrandToken inherits OpenZeppelin ERC20 default decimals = 18.
    let monthly_emission = monthly_emission_raw as u128 * 10u128.pow(18);

    let stable_addr_str = token
        .stable_token_address
        .as_deref()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| TokenError::DeployFailed("Stable token not configured".to_string()))?;
    let stable_addr: Address = stable_addr_str
        .parse()
        .map_err(|e| TokenError::DeployFailed(format!("Invalid stable token address: {e}")))?;

    let distribution_wallets: Vec<Address> = token
        .distribution_slots
        .iter()
        .map(|s| {
            s.wallet
                .parse::<Address>()
                .map_err(|e| TokenError::DeployFailed(format!("Invalid slot wallet: {e}")))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let distribution_bps: Vec<u16> = token.distribution_slots.iter().map(|s| s.bps).collect();

    // Mark as deploying before starting
    let now_pre = crate::common::utils::time_utils::get_now();
    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_deploying(true)
        .with_updated_at(now_pre)
        .execute(cli)
        .await?;

    // Use start_month from token config if set (e.g. "2026-03"),
    // otherwise default to 1st of current month.
    let start_timestamp = {
        let fallback = || {
            let now_utc = chrono::Utc::now();
            let y = now_utc.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
            let m = now_utc.format("%m").to_string().parse::<u32>().unwrap_or(1);
            chrono::NaiveDate::from_ymd_opt(y, m, 1)
                .unwrap_or(chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
        };
        let date = token
            .start_month
            .as_deref()
            .and_then(|s| {
                let parts: Vec<&str> = s.split('-').collect();
                if parts.len() == 2 {
                    let y = parts[0].parse::<i32>().ok()?;
                    let m = parts[1].parse::<u32>().ok()?;
                    chrono::NaiveDate::from_ymd_opt(y, m, 1)
                } else {
                    None
                }
            })
            .unwrap_or_else(fallback);
        date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as u64
    };

    let deploy_result = crate::common::blockchain::deploy_brand_system(
        chain_id,
        &token.name,
        &token.symbol,
        monthly_emission,
        token.decay_rate_bps,
        stable_addr,
        distribution_wallets,
        distribution_bps,
        start_timestamp,
    )
    .await;

    let deployment = match deploy_result {
        Ok(d) => d,
        Err(e) => {
            // Reset deploying flag on failure
            let _ = ProjectToken::updater(token.pk.clone(), token.sk.clone())
                .with_deploying(false)
                .with_updated_at(crate::common::utils::time_utils::get_now())
                .execute(cli)
                .await;
            return Err(TokenError::DeployFailed(e).into());
        }
    };

    let token_addr_str = format!("{:?}", deployment.token_address);
    let treasury_addr_str = format!("{:?}", deployment.treasury_address);
    let multisig_addr_str = format!("{:?}", deployment.multisig_address);
    let stable_addr_out = format!("{:?}", deployment.stable_token_address);
    let token_tx_str = format!("{:?}", deployment.token_tx_hash);
    let treasury_tx_str = format!("{:?}", deployment.treasury_tx_hash);
    let multisig_tx_str = format!("{:?}", deployment.multisig_tx_hash);

    let now = crate::common::utils::time_utils::get_now();
    let treasury_reserve_bps =
        (project.treasury_reserve_rate.clamp(0.0, 1.0) * 10000.0).round() as u64;

    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_contract_address(token_addr_str.clone())
        .with_treasury_contract_address(treasury_addr_str.clone())
        .with_multisig_address(multisig_addr_str.clone())
        .with_stable_token_address(stable_addr_out.clone())
        .with_chain_id(chain_id)
        .with_deployment_tx_hash(token_tx_str.clone())
        .with_treasury_deployment_tx_hash(treasury_tx_str.clone())
        .with_multisig_deployment_tx_hash(multisig_tx_str.clone())
        .with_treasury_reserve_bps(treasury_reserve_bps)
        .with_deploying(false)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    let mut updated = token;
    updated.contract_address = Some(token_addr_str);
    updated.treasury_contract_address = Some(treasury_addr_str);
    updated.multisig_address = Some(multisig_addr_str);
    updated.stable_token_address = Some(stable_addr_out);
    updated.chain_id = Some(chain_id);
    updated.deployment_tx_hash = Some(token_tx_str);
    updated.treasury_deployment_tx_hash = Some(treasury_tx_str);
    updated.multisig_deployment_tx_hash = Some(multisig_tx_str);
    updated.treasury_reserve_bps = treasury_reserve_bps;
    updated.deploying = false;
    updated.updated_at = now;

    Ok(updated.into())
}
