use crate::common::{ListResponse, ProjectPartition, Result};
use crate::features::points::PointTransactionResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::points::PointTransaction;

#[get("/v1/projects/:project_id/points/:meta_user_id/transactions", _auth: ProjectAuth)]
pub async fn list_user_transactions_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    limit: i32,
    bookmark: Option<String>,
    date: String,
) -> Result<ListResponse<PointTransactionResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let opt = PointTransaction::opt_with_bookmark(bookmark)
        .limit(limit)
        .sk(date);

    let (transactions, bookmark) =
        PointTransaction::find_by_meta_user(cli, &meta_user_id, opt).await?;
    let items: Vec<PointTransactionResponse> =
        transactions.into_iter().map(|tx| tx.into()).collect();

    Ok((items, bookmark).into())
}
