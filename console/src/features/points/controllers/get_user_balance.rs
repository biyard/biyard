use crate::common::{ProjectPartition, Result};
use crate::features::points::PointBalanceResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::points::{MonthlyPointAggregation, PointBalance};
#[cfg(feature = "server")]
use crate::features::tokens::ProjectToken;

#[get("/v1/projects/:project_id/points/:meta_user_id?month", auth: ProjectViewerAuth)]
pub async fn get_user_balance_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    month: Option<String>,
) -> Result<PointBalanceResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;
    let date = month.unwrap_or_else(crate::common::utils::time_utils::timestamp_to_yyyy_mm);

    let (pb_pk, pb_sk) = PointBalance::keys(project.pk.clone(), meta_user_id.clone(), date.clone());
    let balance = PointBalance::get(cli, &pb_pk, Some(pb_sk))
        .await?
        .unwrap_or_default();

    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone(), date.clone());
    let project_total_points = if let Ok(Some(aggregation)) =
        MonthlyPointAggregation::get(cli, &agg_pk, Some(agg_sk)).await
    {
        aggregation.awarded_points
    } else {
        0
    };

    // Calculate the actual user pool token supply for the requested month,
    // factoring in decay and brand allocation. This replaces the static
    // project.monthly_token_supply so that external services (e.g. ratel)
    // can show accurate exchange rates without extra API calls.
    let monthly_token_supply = compute_month_user_pool(cli, &project.pk, &date).await;

    Ok(PointBalanceResponse {
        project_id: project.pk,
        meta_user_id,
        month: date,
        balance: balance.balance,
        total_earned: balance.total_earned,
        total_spent: balance.total_spent,
        updated_at: balance.updated_at,
        project_total_points,
        monthly_token_supply,
    })
}

/// Compute the user-claimable token pool for a given month.
/// = monthlyCeiling(monthIndex) * (10000 - brandAllocationBps) / 10000
/// Falls back to 0 if token is not configured.
#[cfg(feature = "server")]
async fn compute_month_user_pool(
    cli: &aws_sdk_dynamodb::Client,
    project_pk: &crate::common::types::Partition,
    month_str: &str,
) -> i64 {
    let (token_pk, token_sk) = ProjectToken::keys(project_pk.clone());
    let token = match ProjectToken::get(cli, &token_pk, Some(token_sk)).await {
        Ok(Some(t)) if t.monthly_emission > 0 => t,
        _ => return 0,
    };

    let emission = token.monthly_emission.max(0) as u64;
    let decay_bps = token.decay_rate_bps;
    let total_slot_bps: u64 = token.distribution_slots.iter().map(|s| s.bps as u64).sum();
    let user_pool_bps = 10000u64.saturating_sub(total_slot_bps);

    let month_index = crate::common::utils::time_utils::month_index(
        month_str,
        &token.start_month,
        token.created_at,
    );

    let mut ceiling = emission as u128;
    for _ in 0..month_index {
        ceiling = ceiling * (10000 - decay_bps as u128) / 10000;
    }
    let user_pool = ceiling * user_pool_bps as u128 / 10000;
    user_pool as i64
}
