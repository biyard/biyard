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

#[post("/v1/accounts/signin", session: Extension<tower_sessions::Session>)]
pub async fn signin_handler(email: String, password: String) -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let Extension(session) = session;

    let (accounts, _) =
        Account::find_by_email(cli, &email, AccountQueryOption::builder().limit(1)).await?;

    let Some(mut account) = accounts.into_iter().next() else {
        return Err(AccountError::InvalidCredentials.into());
    };

    let effective_scheme = if matches!(account.password_scheme, PasswordScheme::LegacySha3)
        && crate::common::utils::user_password_utils::is_bcrypt_hash(&account.password)
    {
        PasswordScheme::BcryptV1
    } else {
        account.password_scheme
    };

    let is_valid = match effective_scheme {
        PasswordScheme::LegacySha3 => {
            crate::common::utils::password_utils::verify_secret_for_lookup(
                &password,
                &account.password,
            )
        }
        PasswordScheme::BcryptV1 => crate::common::utils::user_password_utils::verify_password(
            &password,
            &account.password,
        )?,
    };

    if !is_valid {
        return Err(AccountError::InvalidCredentials.into());
    }

    if matches!(effective_scheme, PasswordScheme::LegacySha3)
        || account.password_scheme != effective_scheme
    {
        let password_hash = crate::common::utils::user_password_utils::hash_password(&password)?;
        account = Account::updater(account.pk.clone(), account.sk.clone())
            .with_password(password_hash)
            .with_password_scheme(PasswordScheme::BcryptV1)
            .with_updated_at(crate::common::utils::time_utils::get_now())
            .execute(cli)
            .await?;
    }

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    let (account, _) = ensure_current_enterprise_for_account(cli, &account).await?;

    Ok(account.into())
}
