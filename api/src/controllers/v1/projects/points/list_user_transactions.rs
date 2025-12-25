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
    Extension(project): Extension<Project>,
    Path(ProjectPointPathParam { meta_user_id }): Path<ProjectPointPathParam>,
    Query(ListUserTransactionsRequest { pagination, date }): Query<ListUserTransactionsRequest>,
) -> Result<Json<ListResponse<PointTransactionResponse>>> {
    let pk = PointTransaction::generate_pk_for_find_by_user(project.pk, meta_user_id);
    let sk = PointTransaction::generate_sk_for_find_by_user(date.clone());
    let opt = PointTransaction::opt_with_bookmark(pagination.bookmark)
        .limit(pagination.limit)
        .sk(sk);

    let (transactions, bookmark): (Vec<PointTransaction>, Option<String>) =
        PointTransaction::find_by_user(&cli, pk, opt).await?;

    let items = transactions.into_iter().map(Into::into).collect();

    Ok(Json(ListResponse { items, bookmark }))
}
