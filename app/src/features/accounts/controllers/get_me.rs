use dioxus::prelude::get;
use crate::common::Result;
use crate::features::accounts::{Account, AccountResponse};

#[get("/v1/accounts/me", account: Account)]
pub async fn get_me_handler() -> Result<AccountResponse> {
    Ok(account.into())
}
