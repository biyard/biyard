use crate::common::{ProjectPartition, Result};
use crate::features::points::PointBalanceResponse;
use dioxus::prelude::get;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::points::{MonthlyPointAggregation, PointBalance};

#[get("/v1/projects/:project_id/points/:meta_user_id", auth: ProjectAuth)]
pub async fn get_user_balance_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    date: String,
) -> Result<PointBalanceResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (pb_pk, pb_sk) = PointBalance::keys(project.pk.clone(), meta_user_id.clone(), date.clone());
    let balance = PointBalance::get(cli, &pb_pk, Some(pb_sk))
        .await?
        .unwrap_or_default();

    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone(), date.clone());
    let project_total_points = if let Ok(Some(aggregation)) =
        MonthlyPointAggregation::get(cli, &agg_pk, Some(agg_sk)).await
    {
        aggregation.supplied_points
    } else {
        0
    };

    Ok(PointBalanceResponse {
        project_id: project.pk,
        meta_user_id,
        month: date,
        balance: balance.balance,
        total_earned: balance.total_earned,
        total_spent: balance.total_spent,
        updated_at: balance.updated_at,
        project_total_points,
        monthly_token_supply: project.monthly_token_supply,
    })
}
