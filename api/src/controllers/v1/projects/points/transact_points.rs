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
            _ => {
                todo!()
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
    let user_pk = Partition::MetaUser(to.clone());
    let pk = CompositePartition(project.pk.clone(), user_pk);
    let sk = EntityType::Month(month.clone());

    let now = time_utils::get_now();

    let point_balance = PointBalance::updater(pk, sk)
        .increase_balance(amount)
        .increase_total_earned(amount)
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

    let (pk, sk) =
        MonthlyPointAggregation::keys(project.pk.clone().into(), to.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::updater(pk, sk)
        .increase_awarded_points(amount)
        .increase_supplied_points(amount)
        .with_updated_at(now);

    // Business logic for awarding points can be added here
    // For now, we just log the action

    vec![
        point_balance.transact_upsert_item(),
        transaction.create_transact_write_item(),
        aggregation.transact_upsert_item(),
    ]
}
