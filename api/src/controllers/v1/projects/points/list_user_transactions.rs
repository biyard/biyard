use crate::features::points::*;
use crate::features::projects::*;
use crate::utils::time_utils::timestamp_to_yyyy_mm;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ListUserTransactionsRequest {
    #[serde(flatten)]
    pub pagination: Pagination,

    #[schemars(description = "Date in YYYY-MM format/YYYY format")]
    #[serde(default = "time_utils::timestamp_to_yyyy_mm")]
    pub date: String,
}

pub async fn list_user_transactions_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(_): Extension<Project>,
    Path(ProjectUserPathParam { meta_user_id, .. }): ProjectUserPath,
    Query(ListUserTransactionsRequest { pagination, date }): Query<ListUserTransactionsRequest>,
) -> Result<Json<ListResponse<PointTransactionResponse>>> {
    let opt = PointTransaction::opt_with_bookmark(pagination.bookmark)
        .limit(pagination.limit)
        .sk(date);
    let (transactions, bookmark) =
        PointTransaction::find_by_meta_user(&cli, &meta_user_id, opt).await?;

    let items = transactions.into_iter().map(Into::into).collect();

    Ok(Json(ListResponse { items, bookmark }))
}
