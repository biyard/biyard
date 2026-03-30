use crate::common::{CommonConfig, ProjectAuth, ProjectPartition, Result};
use crate::features::points::{
    MonthlyPointAggregation, MonthlyPointAggregationResponse, PointError,
};
use dioxus::prelude::get;

#[get("/v1/projects/:project_id/points", auth: ProjectAuth)]
pub async fn get_point_aggregation_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    date: String,
) -> Result<MonthlyPointAggregationResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (pk, sk) = MonthlyPointAggregation::keys(auth.project.pk, date);
    let res = MonthlyPointAggregation::get(cli, &pk, Some(sk))
        .await?
        .ok_or(PointError::PointAggregationNotFound)?;

    Ok(res.into())
}
