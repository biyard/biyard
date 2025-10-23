pub mod signup_account;

use signup_account::signup_account_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/", post(signup_account_handler)))
}
