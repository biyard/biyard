use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetPointAggregationResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn get_point_aggregation_handler(
    State(AppState { cli: _, .. }): State<AppState>,
    Query(Pagination {
        bookmark: _,
        limit: _,
    }): PaginationQuery,
) -> Result<Json<ListResponse<GetPointAggregationResponse>>> {
    // TODO: Implement the handler logic here

    unimplemented!()
}
