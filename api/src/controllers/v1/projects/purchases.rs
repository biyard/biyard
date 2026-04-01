use crate::features::points::*;
use crate::features::projects::*;
use crate::*;

pub async fn create_purchase_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Json(req): Json<PurchaseRequest>,
) -> Result<Json<PurchaseResponse>> {
    debug!(
        "Processing purchase for project {:?}, user: {}, amount: {}",
        project.pk, req.meta_user_id, req.amount
    );

    if req.amount <= 0 {
        return Err(Error::InvalidPointAmount);
    }

    if req.reward_rate < 0.0 {
        return Err(Error::InvalidExchangeRate);
    }

    let reward_points = ((req.amount as f64) * req.reward_rate / 100.0) as i64;
    let treasury_contribution = req.amount;
    let month = time_utils::timestamp_to_yyyy_mm();
    let now = time_utils::get_now();

    let description = Some(format!("Purchase reward: {}", req.item_name));

    let (bal_pk, bal_sk) = PointBalance::keys(
        project.pk.clone(),
        req.meta_user_id.clone(),
        month.clone(),
    );

    let point_balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(req.meta_user_id.clone())
        .with_month(month.clone())
        .with_total_spent(0)
        .increase_total_earned(reward_points)
        .increase_balance(reward_points)
        .with_updated_at(now);

    let transaction = PointTransaction::new(
        project.pk.clone(),
        req.meta_user_id.clone(),
        month.clone(),
        TransactionType::Award,
        reward_points,
        None,
        description,
    );

    let (agg_pk, agg_sk) =
        MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(agg_pk, agg_sk)
        .increase_awarded_points(reward_points)
        .increase_supplied_points(reward_points)
        .with_updated_at(now);

    let txs = vec![
        point_balance.transact_upsert_item(),
        transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ];

    transact_write_items!(&cli, txs)?;

    Ok(Json(PurchaseResponse {
        purchase_amount: req.amount,
        reward_points,
        treasury_contribution,
    }))
}
