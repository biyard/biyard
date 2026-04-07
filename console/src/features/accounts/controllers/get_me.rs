use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::features::accounts::Account;
#[cfg(feature = "server")]
use crate::features::enterprises::controllers::ensure_current_enterprise_for_account;

#[get("/v1/accounts/me", account: Account)]
pub async fn get_me_handler() -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let (account, _) = ensure_current_enterprise_for_account(cli, &account).await?;
    Ok(account.into())
}
