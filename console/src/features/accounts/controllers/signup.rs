use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::SESSION_KEY_ACCOUNT_ID;
#[cfg(feature = "server")]
use crate::common::{CommonConfig, Extension, OrganizationRole, Partition};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountError, AccountQueryOption, PasswordScheme};
#[cfg(feature = "server")]
use crate::features::enterprises::Enterprise;

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
    let mut account = Account::new(name, email, password_hash, PasswordScheme::BcryptV1);

    let enterprise_pk = Partition::Enterprise(uuid::Uuid::now_v7().to_string());
    let enterprise = Enterprise::new(
        enterprise_pk.clone(),
        account.pk.clone(),
        format!("{} Personal", account.name),
    );

    account.enterprise_id = enterprise_pk;
    account.organization_role = OrganizationRole::Owner;

    crate::transact_write!(
        cli,
        enterprise.create_transact_write_item(),
        account.create_transact_write_item(),
    )?;

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    Ok(account.into())
}
