use crate::common::{EntityType, Result};
use crate::features::accounts::AccountResponse;
use dioxus::prelude::post;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, Extension};
#[cfg(feature = "server")]
use crate::features::accounts::Account;

#[post("/v1/accounts/withdrawal", account: Account, session: Extension<tower_sessions::Session>)]
pub async fn withdrawal_handler() -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let Extension(session) = session;

    let deleted_account = Account::delete(cli, account.pk, Some(EntityType::Account)).await?;

    let _ = session.flush().await;

    Ok(deleted_account.into())
}
