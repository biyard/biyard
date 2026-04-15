use crate::common::{ListResponse, ProjectPartition, Result};
use crate::features::points::PointTransactionResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::points::PointTransaction;

#[api_doc_macros::api_doc(group = "Points", summary = "List all transactions")]
#[get(
    "/v1/projects/:project_id/points/transactions?limit&bookmark&newest_first",
    auth: ProjectViewerAuth
)]
pub async fn list_transactions_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    limit: i32,
    bookmark: Option<String>,
    newest_first: Option<bool>,
) -> Result<ListResponse<PointTransactionResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    // DynamoDB orders results by the GSI sort key. `gsi3` uses
    // `created_at` with the `TS` prefix, so `scan_index_forward(false)`
    // gives newest-first and `scan_index_forward(true)` gives
    // oldest-first without any application-side sorting.
    let scan_forward = !newest_first.unwrap_or(true);
    let opt = PointTransaction::opt_with_bookmark(bookmark)
        .limit(limit)
        .scan_index_forward(scan_forward);

    let (transactions, bookmark) =
        PointTransaction::find_by_project_time(cli, &auth.project.pk, opt).await?;
    let items: Vec<PointTransactionResponse> =
        transactions.into_iter().map(|tx| tx.into()).collect();

    Ok((items, bookmark).into())
}
