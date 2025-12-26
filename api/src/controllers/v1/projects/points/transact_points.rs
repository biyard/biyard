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
) -> Result<Json<Vec<TransactPointsResponse>>> {
    debug!("Awarding points in project: {:?}", project);

    let mut txs = vec![];
    let mut responses = vec![];

    for req in req {
        // Validate the request
        req.validate()?;

        let TransactPointsRequest {
            month,
            description,
            tx,
        } = req;

        let (write_items, tx_responses) = match tx {
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
        txs.extend(write_items);
        responses.extend(tx_responses);
    }

    transact_write_items!(&cli, txs)?;

    Ok(Json(responses))
}

fn award_points(
    project: &Project,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    debug!(
        "Awarding {} points to {} in project {}",
        amount, to, project.name
    );

    // Increase Monthly Points Supply
    let now = time_utils::get_now();

    let (bal_pk, bal_sk) = PointBalance::keys(project.pk.clone(), to.clone(), month.clone());

    let point_balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(to.clone())
        .with_month(month.clone())
        .with_total_spent(0) // Initialize if not exists
        .increase_total_earned(amount)
        .increase_balance(amount)
        .with_updated_at(now);

    let transaction = PointTransaction::new(
        project.pk.clone(),
        to.clone(),
        month.clone(),
        TransactionType::Award,
        amount,
        None,
        description,
    );

    let transaction_id = transaction.sk.to_string();

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_awarded_points(amount)
        .increase_supplied_points(amount)
        .with_updated_at(now);

    let response = TransactPointsResponse {
        transaction_id,
        month: month.clone(),
        meta_user_id: to,
        transaction_type: "Award".to_string(),
        amount,
    };

    (
        vec![
            point_balance.transact_upsert_item(),
            transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![response],
    )
}

fn deduct_points(
    project: &Project,
    from: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    debug!(
        "Deducting {} points from {} in project {}",
        amount, from, project.name
    );

    let now = time_utils::get_now();

    // Decrease balance and increase total_spent
    let (bal_pk, bal_sk) = PointBalance::keys(project.pk.clone(), from.clone(), month.clone());

    let point_balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .with_total_earned(0) // Initialize if not exists
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

    let transaction_id = transaction.sk.to_string();

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_deducted_points(amount)
        .decrease_supplied_points(amount)
        .with_updated_at(now);

    let response = TransactPointsResponse {
        transaction_id,
        month: month.clone(),
        meta_user_id: from,
        transaction_type: "Deduct".to_string(),
        amount,
    };

    (
        vec![
            point_balance.transact_upsert_item(),
            transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![response],
    )
}

fn transfer_points(
    project: &Project,
    from: String,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    debug!(
        "Transferring {} points from {} to {} in project {}",
        amount, from, to, project.name
    );

    let now = time_utils::get_now();

    // Deduct from sender
    let (from_bal_pk, from_bal_sk) =
        PointBalance::keys(project.pk.clone(), from.clone(), month.clone());

    let from_balance = PointBalance::updater(from_bal_pk, from_bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .with_total_earned(0) // Initialize if not exists
        .decrease_balance(amount)
        .increase_total_spent(amount)
        .with_updated_at(now);

    // Add to recipient
    let (to_bal_pk, to_bal_sk) = PointBalance::keys(project.pk.clone(), to.clone(), month.clone());

    let to_balance = PointBalance::updater(to_bal_pk, to_bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(to.clone())
        .with_month(month.clone())
        .with_total_spent(0) // Initialize if not exists
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

    let from_transaction_id = from_transaction.sk.to_string();

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

    let to_transaction_id = to_transaction.sk.to_string();

    // Update aggregation for sender
    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(agg_pk, agg_sk)
        .increase_traded_points(amount)
        .with_updated_at(now);

    let from_response = TransactPointsResponse {
        transaction_id: from_transaction_id,
        month: month.clone(),
        meta_user_id: from,
        transaction_type: "Transfer".to_string(),
        amount: -amount,
    };

    let to_response = TransactPointsResponse {
        transaction_id: to_transaction_id,
        month: month.clone(),
        meta_user_id: to,
        transaction_type: "Transfer".to_string(),
        amount,
    };

    (
        vec![
            from_balance.transact_upsert_item(),
            to_balance.transact_upsert_item(),
            from_transaction.create_transact_write_item(),
            to_transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![from_response, to_response],
    )
}

fn exchange_points(
    project: &Project,
    from: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    debug!(
        "Exchanging {} points from {} to tokens in project {}",
        amount, from, project.name
    );

    let now = time_utils::get_now();

    // Decrease balance and increase total_spent
    let (bal_pk, bal_sk) = PointBalance::keys(project.pk.clone(), from.clone(), month.clone());

    let point_balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .with_total_earned(0) // Initialize if not exists
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

    let transaction_id = transaction.sk.to_string();

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_exchanged_points(amount)
        .with_updated_at(now);

    let response = TransactPointsResponse {
        transaction_id,
        month: month.clone(),
        meta_user_id: from,
        transaction_type: "Exchange".to_string(),
        amount,
    };

    (
        vec![
            point_balance.transact_upsert_item(),
            transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![response],
    )
}
