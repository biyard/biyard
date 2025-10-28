pub mod get_current_account;
pub mod signin_account;
pub mod signout_account;
pub mod signup_account;
pub mod withdrawal_account;

#[cfg(test)]
pub mod tests;

use get_current_account::get_current_account_handler;
use signin_account::signin_account_handler;
use signout_account::signout_account_handler;
use signup_account::signup_account_handler;
use withdrawal_account::withdrawal_account_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/signup", post(signup_account_handler))
        .route("/signin", post(signin_account_handler))
        .route("/signout", post(signout_account_handler))
        .route("/withdrawal", post(withdrawal_account_handler))
        .route("/me", get(get_current_account_handler)))
}
