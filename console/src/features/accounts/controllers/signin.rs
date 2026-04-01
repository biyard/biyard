use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::SESSION_KEY_ACCOUNT_ID;
#[cfg(feature = "server")]
use crate::common::{CommonConfig, Extension};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountError, AccountQueryOption};

#[post("/v1/accounts/signin", session: Extension<tower_sessions::Session>)]
pub async fn signin_handler(email: String, password: String) -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let Extension(session) = session;

    let hashed_password = crate::common::utils::password_utils::hash_password(&password);

    let (accounts, _) = Account::find_by_email_and_password(
        cli,
        &email,
        AccountQueryOption::builder().limit(1).sk(hashed_password),
    )
    .await?;

    if accounts.is_empty() {
        return Err(AccountError::InvalidCredentials.into());
    }

    let account = &accounts[0];

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    Ok(account.to_owned().into())
}
