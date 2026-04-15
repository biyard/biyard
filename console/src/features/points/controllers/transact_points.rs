use crate::common::{ProjectPartition, Result};
use crate::features::points::{TransactPointsRequest, TransactPointsResponse};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::points::{
    MonthlyPointAggregation, PointBalance, PointTransaction, Transaction, TransactionType,
};
#[cfg(feature = "server")]
use crate::features::projects::Project;
#[cfg(feature = "server")]
use aws_sdk_dynamodb::types::TransactWriteItem;

#[post("/v1/projects/:project_id/points", auth: ProjectAuth)]
pub async fn transact_points_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    transactions: Vec<TransactPointsRequest>,
) -> Result<Vec<TransactPointsResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let mut txs: Vec<TransactWriteItem> = vec![];
    let mut responses = vec![];

    for tx_req in transactions {
        let TransactPointsRequest {
            month,
            description,
            tx,
        } = tx_req;

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
            Transaction::Exchange { .. } => {
                return Err(crate::features::points::PointError::InvalidTransaction(
                    "Exchange must be performed via mint_token".to_string(),
                )
                .into());
            }
        };
        txs.extend(write_items);
        responses.extend(tx_responses);
    }

    crate::transact_write_items!(cli, txs)?;

    Ok(responses)
}

#[cfg(feature = "server")]
fn award_points(
    project: &Project,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    let now = crate::common::utils::time_utils::get_now();
    let (bal_pk, bal_sk) = PointBalance::keys(project.pk.clone(), to.clone(), month.clone());

    let point_balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(to.clone())
        .with_month(month.clone())
        .with_total_spent(0)
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

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.clone());
    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_awarded_points(amount)
        .increase_supplied_points(amount)
        .with_updated_at(now);

    (
        vec![
            point_balance.transact_upsert_item(),
            transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![TransactPointsResponse {
            transaction_id,
            month,
            meta_user_id: to,
            transaction_type: "Award".to_string(),
            amount,
        }],
    )
}

#[cfg(feature = "server")]
fn deduct_points(
    project: &Project,
    from: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    let now = crate::common::utils::time_utils::get_now();
    let (bal_pk, bal_sk) = PointBalance::keys(project.pk.clone(), from.clone(), month.clone());

    let point_balance = PointBalance::updater(bal_pk, bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .with_total_earned(0)
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

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.clone());
    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_deducted_points(amount)
        .decrease_supplied_points(amount)
        .with_updated_at(now);

    (
        vec![
            point_balance.transact_upsert_item(),
            transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![TransactPointsResponse {
            transaction_id,
            month,
            meta_user_id: from,
            transaction_type: "Deduct".to_string(),
            amount,
        }],
    )
}

#[cfg(feature = "server")]
fn transfer_points(
    project: &Project,
    from: String,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> (Vec<TransactWriteItem>, Vec<TransactPointsResponse>) {
    let now = crate::common::utils::time_utils::get_now();

    let (from_bal_pk, from_bal_sk) =
        PointBalance::keys(project.pk.clone(), from.clone(), month.clone());
    let from_balance = PointBalance::updater(from_bal_pk, from_bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(from.clone())
        .with_month(month.clone())
        .with_total_earned(0)
        .decrease_balance(amount)
        .increase_total_spent(amount)
        .with_updated_at(now);

    let (to_bal_pk, to_bal_sk) = PointBalance::keys(project.pk.clone(), to.clone(), month.clone());
    let to_balance = PointBalance::updater(to_bal_pk, to_bal_sk)
        .with_project_id(project.pk.clone())
        .with_meta_user_id(to.clone())
        .with_month(month.clone())
        .with_total_spent(0)
        .increase_balance(amount)
        .increase_total_earned(amount)
        .with_updated_at(now);

    let from_transaction = PointTransaction::new(
        project.pk.clone(),
        from.clone(),
        month.clone(),
        TransactionType::Transfer,
        -amount,
        Some(to.clone()),
        description.clone(),
    );
    let from_transaction_id = from_transaction.sk.to_string();

    let to_transaction = PointTransaction::new(
        project.pk.clone(),
        to.clone(),
        month.clone(),
        TransactionType::Transfer,
        amount,
        Some(from.clone()),
        description,
    );
    let to_transaction_id = to_transaction.sk.to_string();

    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.clone());
    let aggregation = MonthlyPointAggregation::updater(agg_pk, agg_sk)
        .increase_traded_points(amount)
        .with_updated_at(now);

    (
        vec![
            from_balance.transact_upsert_item(),
            to_balance.transact_upsert_item(),
            from_transaction.create_transact_write_item(),
            to_transaction.create_transact_write_item(),
            aggregation.transact_upsert_item(),
        ],
        vec![
            TransactPointsResponse {
                transaction_id: from_transaction_id,
                month: month.clone(),
                meta_user_id: from,
                transaction_type: "Transfer".to_string(),
                amount: -amount,
            },
            TransactPointsResponse {
                transaction_id: to_transaction_id,
                month,
                meta_user_id: to,
                transaction_type: "Transfer".to_string(),
                amount,
            },
        ],
    )
}
