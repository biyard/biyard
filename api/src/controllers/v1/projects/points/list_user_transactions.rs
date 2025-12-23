use crate::features::points::*;
use crate::features::projects::*;
use crate::utils::time_utils::timestamp_to_yyyy_mm;
use crate::*;

pub async fn list_user_transactions_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(ProjectPointPathParam {
        meta_user_id,
        month,
    }): Path<ProjectPointPathParam>,
    Query(params): PaginationQuery,
) -> Result<Json<ListResponse<PointTransactionResponse>>> {
    let month = month.unwrap_or(timestamp_to_yyyy_mm());
    let pk = PointTransaction::generate_pk_for_find_by_user(project.pk, meta_user_id);
    let sk = PointTransaction::generate_sk_for_find_by_user(month);
    let opt = PointTransactionQueryOption::builder()
        .limit(params.limit)
        .sk(sk);

    let (transactions, bookmark): (Vec<PointTransaction>, Option<String>) =
        PointTransaction::find_by_user(&cli, pk, opt).await?;

    let items = transactions.into_iter().map(Into::into).collect();

    Ok(Json(ListResponse { items, bookmark }))
}
