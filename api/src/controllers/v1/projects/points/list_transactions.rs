use std::sync::Arc;

use crate::features::accounts::Account;
use crate::features::points::*;
use crate::features::projects::*;
use crate::*;

pub async fn list_transactions_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(_path): ProjectPath,
    Query(params): PaginationQuery,
) -> Result<Json<ListResponse<PointTransactionResponse>>> {
    debug!("Listing transactions for project: {:?}", project);

    params.validate()?;

    // Query transactions by project
    let mut opt = PointTransactionQueryOption::builder().limit(params.limit.unwrap_or(100));

    if let Some(bookmark) = params.bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (transactions, bookmark) =
        PointTransaction::find_by_project(&cli, &project.pk, opt).await?;

    let items: Vec<PointTransactionResponse> =
        transactions.into_iter().map(|tx| tx.into()).collect();

    Ok(Json((items, bookmark).into()))
}
