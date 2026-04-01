use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::get;

#[cfg(feature = "server")]
use crate::features::accounts::Account;

#[get("/v1/accounts/me", account: Account)]
pub async fn get_me_handler() -> Result<AccountResponse> {
    Ok(account.into())
}
