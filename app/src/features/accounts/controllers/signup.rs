use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::post;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, Extension};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountError, AccountQueryOption};
#[cfg(feature = "server")]
use super::SESSION_KEY_ACCOUNT_ID;

#[post("/v1/accounts/signup", session: Extension<tower_sessions::Session>)]
pub async fn signup_handler(
    name: String,
    email: String,
    hashed_password: String,
) -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let Extension(session) = session;

    let (accounts, _) =
        Account::find_by_email(cli, &email, AccountQueryOption::builder().limit(1)).await?;

    if !accounts.is_empty() {
        return Err(AccountError::EmailAlreadyExists.into());
    }

    let hashed_password = crate::common::utils::password_utils::hash_password(&hashed_password);
    let account = Account::new(name, email, hashed_password);

    account.create(cli).await?;

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    Ok(account.into())
}
