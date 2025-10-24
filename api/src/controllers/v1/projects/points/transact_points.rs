use crate::features::accounts::Account;
use crate::features::points::*;
use crate::features::projects::*;
use crate::*;
use chrono::prelude::*;

pub async fn transact_points_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(ProjectPointPathParam { meta_user_id, .. }): ProjectPointPath,
    Json(req): Json<TransactPointsRequest>,
) -> Result<()> {
    debug!("Awarding points in project: {:?}", project);

    // Validate the request
    req.validate()?;

    let TransactPointsRequest {
        amount,
        month,
        description,
        tx_type: _,
    } = req;

    // Determine month
    let user_pk = Partition::MetaUser(meta_user_id.clone());
    let pk = CompositePartition(project.pk.clone(), user_pk);
    let month = month.unwrap_or(time_utils::timestamp_to_yyyy_mm());
    let sk = EntityType::Month(month.clone());

    let now = time_utils::get_now();
    let point_balance = PointBalance::updater(pk, sk)
        .increase_balance(amount)
        .increase_total_earned(amount)
        .with_updated_at(now);

    // Create transaction record
    let transaction = PointTransaction::new(
        project.pk,
        meta_user_id,
        month,
        TransactionType::Award,
        amount,
        None,
        description,
    );

    transact_write!(
        &cli,
        point_balance.transact_write_item(),
        transaction.create_transact_write_item(),
    )?;

    Ok(())
}
