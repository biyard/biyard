use chrono::Months;

use crate::features::{
    points::*,
    projects::{ProjectPath, ProjectPathParam},
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetPointAggregationRequest {
    #[serde(flatten)]
    pub pagination: Pagination,

    #[schemars(description = "Date in YYYY-MM format/YYYY format")]
    #[serde(default = "time_utils::timestamp_to_yyyy_mm")]
    pub date: String,
}

pub async fn get_point_aggregation_handler(
    State(AppState { cli, .. }): State<AppState>,
    Path(ProjectPathParam { project_id }): ProjectPath,
    Query(GetPointAggregationRequest { pagination, date }): Query<GetPointAggregationRequest>,
) -> Result<Json<ListResponse<MonthlyPointAggregationResponse>>> {
    let opt = MonthlyPointAggregation::opt_with_bookmark(pagination.bookmark)
        .limit(pagination.limit)
        .sk(date);

    let pk: Partition = project_id.into();

    let res = MonthlyPointAggregation::find_by_date(&cli, pk, opt)
        .await
        .map(|(res, bm)| {
            (
                res.into_iter()
                    .map(|e| {
                        let e: MonthlyPointAggregationResponse = e.into();
                        e
                    })
                    .collect(),
                bm,
            )
        })?;

    Ok(Json(res.into()))
}
