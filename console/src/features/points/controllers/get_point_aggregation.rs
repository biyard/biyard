use crate::common::{ProjectPartition, Result};
use crate::features::points::MonthlyPointAggregationResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::points::MonthlyPointAggregation;

/// Returns the aggregated point activity for a project for a given month.
///
/// Always returns `200 OK`. If no aggregation row exists yet (e.g. fresh
/// brand with zero point activity), the response is a zero-filled
/// aggregation rather than a 404, so the client can render "No activity
/// this month" cleanly without log noise.
#[api_doc_macros::api_doc(group = "Points", summary = "Get point aggregation", summary_ko = "포인트 집계 조회")]
#[get("/v1/projects/:project_id/points?month", auth: ProjectViewerAuth)]
pub async fn get_point_aggregation_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    month: String,
) -> Result<MonthlyPointAggregationResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (pk, sk) = MonthlyPointAggregation::keys(auth.project.pk, month.clone());
    let res = MonthlyPointAggregation::get(cli, &pk, Some(sk)).await?;

    Ok(res
        .map(Into::into)
        .unwrap_or_else(|| MonthlyPointAggregationResponse::empty(month)))
}
