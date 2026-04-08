use crate::common::{ListResponse, ProjectPartition, Result};
use crate::features::points::PointTransactionResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, CompositePartition, Partition, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::points::PointTransaction;

#[get("/v1/projects/:project_id/points/:meta_user_id/transactions?limit&bookmark&month", auth: ProjectViewerAuth)]
pub async fn list_user_transactions_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    limit: i32,
    bookmark: Option<String>,
    month: Option<String>,
) -> Result<ListResponse<PointTransactionResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let pk = CompositePartition(auth.project.pk, Partition::MetaUser(meta_user_id));

    let sk_prefix = match &month {
        Some(m) => format!("POINT_TRANSACTION#{m}#"),
        None => "POINT_TRANSACTION#".to_string(),
    };
    let opt = PointTransaction::opt_with_bookmark(bookmark)
        .limit(limit)
        .sk(sk_prefix)
        .scan_index_forward(false);

    let (transactions, bookmark) = PointTransaction::query(cli, &pk, opt).await?;
    let items: Vec<PointTransactionResponse> =
        transactions.into_iter().map(|tx| tx.into()).collect();

    Ok((items, bookmark).into())
}
