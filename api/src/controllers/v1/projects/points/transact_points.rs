use crate::features::accounts::Account;
use crate::features::points::*;
use crate::features::projects::*;
use crate::*;
use aws_sdk_dynamodb::types::TransactWriteItem;
use chrono::prelude::*;

pub async fn transact_points_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Json(req): Json<TransactPointsRequest>,
) -> Result<()> {
    debug!("Awarding points in project: {:?}", project);

    // Validate the request
    req.validate()?;

    let TransactPointsRequest {
        month,
        description,
        tx,
    } = req;

    let txs = match tx {
        Transaction::Award { to, amount } => {
            award_points(project, to, amount, month, description).await?
        }
        _ => {
            todo!()
        }
    };

    transact_write_items!(&cli, txs);

    Ok(())
}

async fn award_points(
    project: Project,
    to: String,
    amount: i64,
    month: String,
    description: Option<String>,
) -> Result<Vec<TransactWriteItem>> {
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
        project.pk,
        to,
        month,
        TransactionType::Award,
        amount,
        None,
        description,
    );

    // Business logic for awarding points can be added here
    // For now, we just log the action

    Ok(vec![
        point_balance.transact_upsert_item(),
        transaction.create_transact_write_item(),
    ])
}
