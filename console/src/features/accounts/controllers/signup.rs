use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::SESSION_KEY_ACCOUNT_ID;
#[cfg(feature = "server")]
use crate::common::{CommonConfig, Extension};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountError, AccountQueryOption, PasswordScheme};
#[cfg(feature = "server")]
use crate::features::enterprises::controllers::ensure_current_enterprise_for_account;

#[post("/v1/accounts/signup", session: Extension<tower_sessions::Session>)]
pub async fn signup_handler(
    name: String,
    email: String,
    password: String,
) -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let Extension(session) = session;

    let (accounts, _) =
        Account::find_by_email(cli, &email, AccountQueryOption::builder().limit(1)).await?;

    if !accounts.is_empty() {
        return Err(AccountError::EmailAlreadyExists.into());
    }

    crate::common::utils::user_password_utils::enforce_password_policy(
        &password,
        Some(&email),
        Some(&name),
    )?;

    let password_hash = crate::common::utils::user_password_utils::hash_password(&password)?;
    let account = Account::new(name, email, password_hash, PasswordScheme::BcryptV1);

    account.create(cli).await?;
    let (account, _) = ensure_current_enterprise_for_account(cli, &account).await?;

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    Ok(account.into())
}
