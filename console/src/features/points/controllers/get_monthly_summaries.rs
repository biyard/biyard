use crate::common::{ProjectPartition, Result};
use crate::features::points::MonthlySummariesResponse;
#[cfg(feature = "server")]
use crate::features::points::MonthlySummaryItem;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::points::{MonthlyPointAggregation, PointBalance};
#[cfg(feature = "server")]
use crate::features::tokens::{ProjectToken, TokenError};

#[get("/v1/projects/:project_id/points/:meta_user_id/monthly-summaries", auth: ProjectViewerAuth)]
pub async fn get_monthly_summaries_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
) -> Result<MonthlySummariesResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let current_month = crate::common::utils::time_utils::timestamp_to_yyyy_mm();

    let opt = PointBalance::opt().limit(100);
    let (balances, _) =
        PointBalance::find_by_meta_user(cli, &format!("PB#{meta_user_id}"), opt).await?;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    let emission = token.monthly_emission.max(0) as u64;
    let decay_bps = token.decay_rate_bps;
    let total_slot_bps: u64 = token.distribution_slots.iter().map(|s| s.bps as u64).sum();
    let user_pool_bps = 10000u64.saturating_sub(total_slot_bps);
    let token_start_month = token.start_month.clone();
    let token_created_at = token.created_at;

    let mut months = Vec::new();

    for bal in &balances {
        if bal.project_id != project.pk {
            continue;
        }
        if bal.month == current_month {
            continue;
        }
        if bal.total_earned <= 0 {
            continue;
        }

        let (agg_pk, agg_sk) =
            MonthlyPointAggregation::keys(project.pk.clone(), bal.month.clone());
        let project_total_points =
            if let Ok(Some(agg)) = MonthlyPointAggregation::get(cli, &agg_pk, Some(agg_sk)).await {
                agg.awarded_points
            } else {
                0
            };

        let monthly_token_supply = {
            let month_index = crate::common::utils::time_utils::month_index(
                &bal.month,
                &token_start_month,
                token_created_at,
            );
            let mut ceiling = emission as u128;
            for _ in 0..month_index {
                ceiling = ceiling * (10000 - decay_bps as u128) / 10000;
            }
            (ceiling * user_pool_bps as u128 / 10000) as i64
        };

        let exchanged = bal.balance == 0 && bal.total_spent > 0;

        months.push(MonthlySummaryItem {
            month: bal.month.clone(),
            total_earned: bal.total_earned,
            total_spent: bal.total_spent,
            balance: bal.balance,
            project_total_points,
            monthly_token_supply,
            exchanged,
        });
    }

    months.sort_by(|a, b| b.month.cmp(&a.month));

    Ok(MonthlySummariesResponse { months })
}
