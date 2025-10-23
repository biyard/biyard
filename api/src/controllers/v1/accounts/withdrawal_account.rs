use crate::features::accounts::*;
use crate::utils::password_utils;
use crate::*;

pub async fn withdrawal_account_handler(
    State(AppState { cli, .. }): State<AppState>,
    NoApi(account): NoApi<Account>,
) -> Result<Json<AccountResponse>> {
    // Delete the account
    let deleted_account =
        Account::delete(&cli, account.pk.to_string(), Some(account.sk.to_string())).await?;

    debug!(
        "Successfully withdrew account for email: {}",
        deleted_account.email
    );

    Ok(Json(deleted_account.into()))
}
