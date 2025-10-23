pub mod signin_account;
pub mod signup_account;
pub mod withdrawal_account;

#[cfg(test)]
pub mod tests;

use signin_account::signin_account_handler;
use signup_account::signup_account_handler;
use withdrawal_account::withdrawal_account_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/signup", post(signup_account_handler))
        .route("/signin", post(signin_account_handler))
        .route("/withdrawal", post(withdrawal_account_handler)))
}
