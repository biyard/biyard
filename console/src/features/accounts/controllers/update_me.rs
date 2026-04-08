use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountError};

/// Update the authenticated user's own profile.
///
/// - `email` is intentionally **not** updatable here. Email is the
///   login identity and changing it requires a verification flow we
///   don't have yet. Treat this handler as name-only for now.
/// - Password changes live in a dedicated handler (not implemented).
/// - This handler is always scoped to the caller — there is no
///   account id in the URL. Callers cannot edit other users' profiles
///   through this route.
#[patch("/v1/accounts/me", account: Account)]
pub async fn update_me_handler(name: Option<String>) -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let mut updater = Account::updater(account.pk.clone(), account.sk.clone());

    if let Some(name) = name {
        let trimmed = name.trim().to_string();
        if trimmed.is_empty() {
            return Err(AccountError::InvalidName.into());
        }
        updater = updater.with_name(trimmed);
    }

    updater = updater.with_updated_at(crate::common::utils::time_utils::get_now());

    let updated = updater.execute(cli).await?;

    Ok(updated.into())
}
