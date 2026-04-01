use crate::common::{ListResponse, ProjectPartition, Result};
use crate::features::points::PointTransactionResponse;
use dioxus::prelude::get;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::points::PointTransaction;

#[get("/v1/projects/:project_id/points/transactions?limit&bookmark", auth: ProjectAuth)]
pub async fn list_transactions_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    limit: i32,
    bookmark: Option<String>,
) -> Result<ListResponse<PointTransactionResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let opt = PointTransaction::opt_with_bookmark(bookmark).limit(limit);

    let (transactions, bookmark) =
        PointTransaction::find_by_project(cli, &auth.project.pk, opt).await?;
    let items: Vec<PointTransactionResponse> =
        transactions.into_iter().map(|tx| tx.into()).collect();

    Ok((items, bookmark).into())
}
