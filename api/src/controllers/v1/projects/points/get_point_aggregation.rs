use chrono::Months;

use crate::features::{
    points::*,
    projects::{ProjectPath, ProjectPathParam},
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetPointAggregationRequest {
    // #[serde(flatten)]
    // pub pagination: Pagination,
    #[schemars(description = "Date in YYYY-MM format/YYYY format")]
    #[serde(default = "time_utils::timestamp_to_yyyy_mm")]
    pub date: String,
}

pub async fn get_point_aggregation_handler(
    State(AppState { cli, .. }): State<AppState>,
    Path(ProjectPathParam { project_id }): ProjectPath,
    Query(GetPointAggregationRequest { date }): Query<GetPointAggregationRequest>,
) -> Result<Json<MonthlyPointAggregationResponse>> {
    let (pk, sk) = MonthlyPointAggregation::keys(ProjectPartition(project_id), date);

    let res = MonthlyPointAggregation::get(&cli, pk, Some(sk))
        .await?
        .ok_or(Error::PointAggregationNotFound)?;

    Ok(Json(res.into()))
}
