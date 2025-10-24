pub mod get_balance;
pub mod list_transactions;
pub mod transact_points;

use crate::*;

pub use get_balance::*;
pub use list_transactions::*;
pub use transact_points::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route(
            "/:meta_user_id",
            get(get_balance_handler).post(transact_points_handler),
        )
        .route("/", get(list_transactions_handler)))
}
