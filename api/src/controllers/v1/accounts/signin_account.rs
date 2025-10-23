use tower_sessions::Session;

use crate::features::accounts::*;
use crate::features::session::SESSION_KEY_ACCOUNT_ID;
use crate::utils::password_utils;
use crate::*;

pub async fn signin_account_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<SigninAccountRequest>,
) -> Result<Json<AccountResponse>> {
    debug!("Handling signin request for email: {}", req.email);

    let hashed_password = password_utils::hash_password(&req.password);

    // Find account by email using the GSI
    let (accounts, _bookmark) = Account::find_by_email_and_password(
        &cli,
        &req.email,
        AccountQueryOption::builder().limit(1).sk(hashed_password),
    )
    .await?;

    // Check if account exists
    if accounts.is_empty() {
        error!("Account not found for email: {}", req.email);
        return Err(Error::InvalidCredentials);
    }

    let account = &accounts[0];

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    debug!(
        "Successfully authenticated account for email: {}",
        req.email
    );

    Ok(Json(account.to_owned().into()))
}
