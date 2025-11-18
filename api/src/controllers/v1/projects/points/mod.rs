mod get_balance;
mod list_transactions;
mod transact_points;

mod get_point_aggregation;
#[cfg(test)]
mod tests;

use crate::*;

use get_balance::*;
use get_point_aggregation::get_point_aggregation_handler;
use list_transactions::*;
use transact_points::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/:meta_user_id", get(get_balance_handler))
        .route("/transactions", get(list_transactions_handler))
        .route(
            "/",
            post(transact_points_handler).get(get_point_aggregation_handler),
        ))
}
