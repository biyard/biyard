use crate::common::{ProjectPartition, Result};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::points::{
    MonthlyPointAggregation, PointBalance, PointTransaction, TransactionType,
};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimableMonth {
    pub month: String,
    pub user_points: i64,
    pub total_points: i64,
    /// Raw ERC-20 token units (includes decimals). String to avoid JSON precision loss.
    pub claimable_tokens: String,
    pub already_claimed: String,
    pub remaining: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimableResponse {
    pub months: Vec<ClaimableMonth>,
}

/// Returns the list of months with claimable tokens for the given user.
#[get("/v1/projects/:project_id/tokens/claimable?meta_user_id", auth: ProjectAdminAuth)]
pub async fn get_claimable_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
) -> Result<ClaimableResponse> {
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

    let total_slot_bps: u16 = token.distribution_slots.iter().map(|s| s.bps).sum();
    let user_pool_bps = 10000u128.saturating_sub(total_slot_bps as u128);

    // Get user's point balances (all months)
    let opt = PointBalance::opt().limit(100);
    let (balances, _) =
        PointBalance::find_by_meta_user(cli, &format!("PB#{meta_user_id}"), opt).await?;

    // On-chain: get currentMonth and already claimed per month
    #[cfg(not(feature = "disable-chain"))]
    let provider = crate::common::blockchain::provider(chain_id)?;
    #[cfg(not(feature = "disable-chain"))]
    let token_addr: ethers::types::Address = contract_address
        .parse()
        .map_err(|e| TokenError::DeployFailed(format!("Invalid token address: {e}")))?;
    #[cfg(not(feature = "disable-chain"))]
    let brand_contract = crate::common::blockchain::BrandTokenContract::new(
        token_addr,
        std::sync::Arc::new(provider),
    );

    // Use on-chain currentMonth (includes advanceMonth offset) for comparison
    #[cfg(not(feature = "disable-chain"))]
    let on_chain_current_month = brand_contract
        .current_month()
        .call()
        .await
        .map(|m| m.as_u64())
        .unwrap_or(0);
    #[cfg(feature = "disable-chain")]
    let on_chain_current_month = crate::common::blockchain::chain_stub::current_month();
    #[cfg(feature = "disable-chain")]
    let _ = (contract_address, chain_id); // silence unused

    let mut months = Vec::new();

    for bal in &balances {
        // Only include balances for this project
        if bal.project_id != project.pk {
            continue;
        }
        // Skip months that haven't ended on-chain yet
        let month_index = month_str_to_index(&bal.month, &token);
        if month_index >= on_chain_current_month {
            continue;
        }
        if bal.total_earned <= 0 {
            continue;
        }

        // Get total points for this month from aggregation
        let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone(), bal.month.clone());
        let agg = MonthlyPointAggregation::get(cli, &agg_pk, Some(agg_sk))
            .await?
            .unwrap_or_default();

        let total_points = agg.awarded_points.max(1);
        let user_points_earned = bal.total_earned;
        let user_points_remaining = bal.balance.max(0);

        let month_index = month_str_to_index(&bal.month, &token);
        #[cfg(not(feature = "disable-chain"))]
        let ceiling = brand_contract
            .monthly_ceiling(ethers::types::U256::from(month_index))
            .call()
            .await
            .map(|v| v.as_u128())
            .unwrap_or(0);
        #[cfg(feature = "disable-chain")]
        let ceiling = crate::common::blockchain::chain_stub::monthly_ceiling(month_index);
        let user_pool = ceiling * user_pool_bps / 10000;

        // Max tokens this user could ever claim (based on total earned points)
        let max_claimable: u128 =
            user_points_earned as u128 * user_pool / total_points.max(1) as u128;

        // Remaining claimable = based on remaining (unspent) points.
        // Points are deducted when a claim signature is issued, so
        // already-claimed share is automatically excluded.
        let remaining: u128 =
            user_points_remaining as u128 * user_pool / total_points.max(1) as u128;

        let already_claimed = max_claimable.saturating_sub(remaining);

        if remaining > 0 || already_claimed > 0 {
            months.push(ClaimableMonth {
                month: bal.month.clone(),
                user_points: user_points_earned,
                total_points,
                claimable_tokens: max_claimable.to_string(),
                already_claimed: already_claimed.to_string(),
                remaining: remaining.to_string(),
            });
        }
    }

    Ok(ClaimableResponse { months })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimSignatureResponse {
    pub month_index: String,
    pub amount: String,
    pub max_claimable: String,
    pub nonce: String,
    pub deadline: String,
    pub signature: String,
    pub contract_address: String,
    pub chain_id: u64,
}

/// Generate a server-signed EIP-712 permit for claiming tokens.
///
/// The server calculates the full claimable amount from the user's remaining
/// point balance — callers do not specify an amount.
#[post("/v1/projects/:project_id/tokens/claim-signature", auth: ProjectAdminAuth)]
pub async fn get_claim_signature_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    month: String,
    wallet_address: String,
) -> Result<ClaimSignatureResponse> {
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
        .ok_or(TokenError::NotDeployed)?
        .to_string();
    let chain_id = token.chain_id.ok_or(TokenError::NotDeployed)?;

    let total_slot_bps: u16 = token.distribution_slots.iter().map(|s| s.bps).sum();
    let user_pool_bps = 10000u128.saturating_sub(total_slot_bps as u128);

    #[cfg(not(feature = "disable-chain"))]
    let provider = crate::common::blockchain::provider(chain_id)?;
    #[cfg(not(feature = "disable-chain"))]
    let token_contract_addr: ethers::types::Address = contract_address
        .parse()
        .map_err(|e| TokenError::DeployFailed(format!("Invalid token address: {e}")))?;
    #[cfg(not(feature = "disable-chain"))]
    let brand_contract = crate::common::blockchain::BrandTokenContract::new(
        token_contract_addr,
        std::sync::Arc::new(provider),
    );
    #[cfg(not(feature = "disable-chain"))]
    let on_chain_current_month = brand_contract
        .current_month()
        .call()
        .await
        .map(|m| m.as_u64())
        .unwrap_or(0);
    #[cfg(feature = "disable-chain")]
    let on_chain_current_month = crate::common::blockchain::chain_stub::current_month();

    let month_index = month_str_to_index(&month, &token);
    if month_index >= on_chain_current_month {
        return Err(TokenError::MintFailed(format!(
            "Month index {month_index} not ended yet (on-chain current: {on_chain_current_month})"
        ))
        .into());
    }

    let (bal_pk, bal_sk) =
        PointBalance::keys(project.pk.clone(), meta_user_id.clone(), month.clone());
    let balance = PointBalance::get(cli, &bal_pk, Some(bal_sk))
        .await?
        .ok_or_else(|| TokenError::MintFailed("No points for this month".to_string()))?;

    let remaining_points = balance.balance.max(0);
    if remaining_points <= 0 {
        return Err(
            TokenError::MintFailed("No remaining points to claim".to_string()).into(),
        );
    }

    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.clone());
    let agg = MonthlyPointAggregation::get(cli, &agg_pk, Some(agg_sk))
        .await?
        .unwrap_or_default();

    let total_points = agg.awarded_points.max(1);

    // monthlyCeiling returns ERC-20 raw units (includes 10^decimals)
    #[cfg(not(feature = "disable-chain"))]
    let ceiling = brand_contract
        .monthly_ceiling(ethers::types::U256::from(month_index))
        .call()
        .await
        .map(|v| v.as_u128())
        .map_err(|e| TokenError::MintFailed(format!("monthlyCeiling call failed: {e}")))?;
    #[cfg(feature = "disable-chain")]
    let ceiling = crate::common::blockchain::chain_stub::monthly_ceiling(month_index);
    let user_pool = ceiling * user_pool_bps / 10000;

    // max_claimable = share based on total earned points
    let max_claimable: u128 =
        balance.total_earned as u128 * user_pool / total_points.max(1) as u128;

    // amount = share based on remaining (unspent) points — claim all remaining
    let amount: u128 = remaining_points as u128 * user_pool / total_points.max(1) as u128;

    if amount == 0 {
        return Err(
            TokenError::MintFailed("Calculated claim amount is zero".to_string()).into(),
        );
    }

    let nonce = uuid::Uuid::now_v7().as_u128() as u64;
    let deadline = crate::common::utils::time_utils::get_now() as u64 / 1000 + 3600; // 1 hour

    let signature = crate::common::blockchain::sign_claim(
        chain_id,
        &contract_address,
        &token.name,
        &wallet_address,
        month_index,
        amount,
        max_claimable,
        nonce,
        deadline,
    )
    .map_err(|e| TokenError::MintFailed(format!("Signing failed: {e}")))?;

    // Record point deduction + exchange in DB atomically.
    let now = crate::common::utils::time_utils::get_now();

    let (bal_pk2, bal_sk2) =
        PointBalance::keys(project.pk.clone(), meta_user_id.clone(), month.clone());
    let point_deduction = PointBalance::updater(bal_pk2, bal_sk2)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(meta_user_id.clone())
        .with_month(month.clone())
        .decrease_balance(remaining_points)
        .increase_total_spent(remaining_points)
        .with_updated_at(now);

    let exchange_tx = PointTransaction::new(
        project.pk.clone(),
        meta_user_id.clone(),
        month.clone(),
        TransactionType::Exchange,
        remaining_points,
        Some(wallet_address.clone()),
        Some(format!("Claim {} tokens (nonce {})", amount, nonce)),
    );

    let (agg_pk2, agg_sk2) = MonthlyPointAggregation::keys(project.pk.clone(), month.clone());
    let aggregation_update = MonthlyPointAggregation::updater(agg_pk2, agg_sk2)
        .increase_exchanged_points(remaining_points)
        .with_updated_at(now);

    crate::transact_write!(
        cli,
        point_deduction.transact_upsert_item(),
        exchange_tx.create_transact_write_item(),
        aggregation_update.transact_upsert_item(),
    )?;

    let sig_hex = format!("0x{}", hex::encode(&signature));

    Ok(ClaimSignatureResponse {
        month_index: month_index.to_string(),
        amount: amount.to_string(),
        max_claimable: max_claimable.to_string(),
        nonce: nonce.to_string(),
        deadline: deadline.to_string(),
        signature: sig_hex,
        contract_address,
        chain_id,
    })
}

#[cfg(feature = "server")]
fn month_str_to_index(month_str: &str, token: &ProjectToken) -> u64 {
    crate::common::utils::time_utils::month_index(month_str, &token.start_month, token.created_at)
}
