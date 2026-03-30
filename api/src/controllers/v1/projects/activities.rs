use crate::features::points::*;
use crate::features::projects::*;
use crate::*;

pub async fn create_activity_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Json(req): Json<ActivityRequest>,
) -> Result<Json<ActivityResponse>> {
    debug!(
        "Processing activity for project {:?}, user: {}, type: {}, value: {}",
        project.pk, req.meta_user_id, req.activity_type, req.value
    );

    if req.value <= 0 {
        return Err(Error::InvalidPointAmount);
    }

    let steps_per_point = if req.steps_per_point > 0 {
        req.steps_per_point
    } else {
        ActivityRequest::default_steps_per_point()
    };

    let points_earned = req.value / steps_per_point;
    let month = time_utils::timestamp_to_yyyy_mm();
    let now = time_utils::get_now();

    let description = Some(format!(
        "Activity reward: {} ({})",
        req.activity_type, req.description
    ));

    let (bal_pk, bal_sk) = PointBalance::keys(
        project.pk.clone(),
        req.meta_user_id.clone(),
        month.clone(),
    );

    let point_balance_updater = PointBalance::updater(bal_pk.clone(), bal_sk.clone())
        .with_project_id(project.pk.clone())
        .with_meta_user_id(req.meta_user_id.clone())
        .with_month(month.clone())
        .with_total_spent(0)
        .increase_total_earned(points_earned)
        .increase_balance(points_earned)
        .with_updated_at(now);

    let transaction = PointTransaction::new(
        project.pk.clone(),
        req.meta_user_id.clone(),
        month.clone(),
        TransactionType::Award,
        points_earned,
        None,
        description,
    );

    let (agg_pk, agg_sk) =
        MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(agg_pk, agg_sk)
        .increase_awarded_points(points_earned)
        .increase_supplied_points(points_earned)
        .with_updated_at(now);

    let txs = vec![
        point_balance_updater.transact_upsert_item(),
        transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ];

    transact_write_items!(&cli, txs)?;

    // Fetch the updated balance for the total_points field
    let updated_balance = PointBalance::get(&cli, &bal_pk, Some(bal_sk))
        .await?
        .unwrap_or_default();

    Ok(Json(ActivityResponse {
        points_earned,
        total_points: updated_balance.balance,
    }))
}
