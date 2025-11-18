use crate::features::accounts::Account;
use crate::features::points::*;
use crate::features::projects::*;
use crate::*;

pub async fn get_balance_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(ProjectPointPathParam { meta_user_id, .. }): Path<ProjectPointPathParam>,
    Query(req): Query<GetPointBalanceRequest>,
) -> Result<Json<ListResponse<PointBalanceResponse>>> {
    debug!(
        "Getting point balance for user {} in project {:?}",
        meta_user_id, project
    );
    let GetPointBalanceRequest { month } = req;

    let user_pk = Partition::MetaUser(meta_user_id);
    let pk = CompositePartition(project.pk, user_pk);
    let month = month.unwrap_or_default();
    let sk = EntityType::Month(month);

    let (balances, bookmark): (Vec<PointBalance>, Option<String>) =
        PointBalance::query_begins_with_sk(&cli, &pk, &sk).await?;

    let items = balances.into_iter().map(Into::into).collect();

    Ok(Json(ListResponse { items, bookmark }))
}
