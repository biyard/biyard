use crate::features::accounts::Account;
use crate::features::points::*;
use crate::features::projects::*;
use crate::utils::time_utils::timestamp_to_yyyy_mm;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Validate)]
pub struct ListTransactionsRequest {
    #[serde(flatten)]
    pub pagination: Pagination,

    #[schemars(description = "Date in YYYY-MM format/YYYY format")]
    pub date: Option<String>,
}

pub async fn list_transactions_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    // Path(_path): ProjectPath,
    Query(ListTransactionsRequest {
        pagination,
        date: _,
    }): Query<ListTransactionsRequest>,
) -> Result<Json<ListResponse<PointTransactionResponse>>> {
    debug!("Listing transactions for project: {:?}", project);

    pagination.validate()?;

    // Query transactions by project
    let opt = PointTransaction::opt_with_bookmark(pagination.bookmark).limit(pagination.limit);

    let (transactions, bookmark) =
        PointTransaction::find_by_project(&cli, &project.pk, opt).await?;

    let items: Vec<PointTransactionResponse> =
        transactions.into_iter().map(|tx| tx.into()).collect();

    Ok(Json((items, bookmark).into()))
}
