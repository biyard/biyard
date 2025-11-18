use crate::features::accounts::Account;
use crate::features::points::*;
use crate::features::projects::*;
use crate::*;
use aws_sdk_dynamodb::types::TransactWriteItem;
use chrono::prelude::*;

pub async fn transact_points_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Json(req): Json<Vec<TransactPointsRequest>>,
) -> Result<()> {
    debug!("Awarding points in project: {:?}", project);

    let mut txs = vec![];
    for req in req {
        // Validate the request
        req.validate()?;

        let TransactPointsRequest {
            month,
            description,
            tx,
        } = req;

        let tx = match tx {
            Transaction::Award { to, amount } => {
                award_points(&project, to, amount, month, description)
            }
            Transaction::Deduct { from, amount } => {
                deduct_points(&project, from, amount, month, description)
            }
            Transaction::Transfer { from, to, amount } => {
                transfer_points(&project, from, to, amount, month, description)
            }
            Transaction::Exchange { from, amount } => {
                exchange_points(&project, from, amount, month, description)
            }
        };
        txs.extend(tx);
    }

    transact_write_items!(&cli, txs);

    Ok(())
}

fn award_points(
    project: &Project,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> Vec<TransactWriteItem> {
    debug!(
        "Awarding {} points to {} in project {}",
        amount, to, project.name
    );

    // Increase Monthly Points Supply
    let now = time_utils::get_now();

    let (bal_pk, bal_sk) = PointBalance::keys(project.pk.clone(), to.clone(), month.clone());

    let balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(to.clone())
        .with_month(month.clone())
        .increase_total_earned(amount)
        .increase_balance(amount)
        .with_updated_at(now);

    let point_balance = balance;

    let transaction = PointTransaction::new(
        project.pk.clone(),
        to.clone(),
        month.clone(),
        TransactionType::Award,
        amount,
        None,
        description,
    );

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_awarded_points(amount)
        .increase_supplied_points(amount)
        .with_updated_at(now);

    vec![
        point_balance.transact_upsert_item(),
        transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ]
}

fn deduct_points(
    project: &Project,
    from: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> Vec<TransactWriteItem> {
    debug!(
        "Deducting {} points from {} in project {}",
        amount, from, project.name
    );

    let user_pk = Partition::MetaUser(from.clone());
    let pk = CompositePartition(project.pk.clone(), user_pk);
    let sk = EntityType::Month(month.clone());

    let now = time_utils::get_now();

    // Decrease balance and increase total_spent
    let point_balance = PointBalance::updater(pk, sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .decrease_balance(amount)
        .increase_total_spent(amount)
        .with_updated_at(now);

    let transaction = PointTransaction::new(
        project.pk.clone(),
        from.clone(),
        month.clone(),
        TransactionType::Deduct,
        amount,
        None,
        description,
    );

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_deducted_points(amount)
        .decrease_supplied_points(amount)
        .with_updated_at(now);

    vec![
        point_balance.transact_upsert_item(),
        transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ]
}

fn transfer_points(
    project: &Project,
    from: String,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> Vec<TransactWriteItem> {
    debug!(
        "Transferring {} points from {} to {} in project {}",
        amount, from, to, project.name
    );

    let now = time_utils::get_now();

    // Deduct from sender
    let from_user_pk = Partition::MetaUser(from.clone());
    let from_pk = CompositePartition(project.pk.clone(), from_user_pk);
    let from_sk = EntityType::Month(month.clone());

    let from_balance = PointBalance::updater(from_pk, from_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .decrease_balance(amount)
        .increase_total_spent(amount)
        .with_updated_at(now);

    // Add to recipient
    let to_user_pk = Partition::MetaUser(to.clone());
    let to_pk = CompositePartition(project.pk.clone(), to_user_pk);
    let to_sk = EntityType::Month(month.clone());

    let to_balance = PointBalance::updater(to_pk, to_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(to.clone())
        .with_month(month.clone())
        .increase_balance(amount)
        .increase_total_earned(amount)
        .with_updated_at(now);

    // Create transaction record for sender (negative amount)
    let from_transaction = PointTransaction::new(
        project.pk.clone(),
        from.clone(),
        month.clone(),
        TransactionType::Transfer,
        -amount, // Negative for sender
        Some(to.clone()),
        description.clone(),
    );

    // Create transaction record for recipient (positive amount)
    let to_transaction = PointTransaction::new(
        project.pk.clone(),
        to.clone(),
        month.clone(),
        TransactionType::Transfer,
        amount, // Positive for recipient
        Some(from.clone()),
        description,
    );

    // Update aggregation for sender
    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(agg_pk, agg_sk)
        .increase_traded_points(amount)
        .with_updated_at(now);

    vec![
        from_balance.transact_upsert_item(),
        to_balance.transact_upsert_item(),
        from_transaction.create_transact_write_item(),
        to_transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ]
}

fn exchange_points(
    project: &Project,
    from: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> Vec<TransactWriteItem> {
    debug!(
        "Exchanging {} points from {} to tokens in project {}",
        amount, from, project.name
    );

    let user_pk = Partition::MetaUser(from.clone());
    let pk = CompositePartition(project.pk.clone(), user_pk);
    let sk = EntityType::Month(month.clone());

    let now = time_utils::get_now();

    // Decrease balance and increase total_spent
    let point_balance = PointBalance::updater(pk, sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .decrease_balance(amount)
        .increase_total_spent(amount)
        .with_updated_at(now);

    let transaction = PointTransaction::new(
        project.pk.clone(),
        from.clone(),
        month.clone(),
        TransactionType::Exchange,
        amount,
        None,
        description,
    );

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_exchanged_points(amount)
        .with_updated_at(now);

    vec![
        point_balance.transact_upsert_item(),
        transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ]
}
