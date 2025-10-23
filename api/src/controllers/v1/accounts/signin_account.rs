use crate::features::accounts::*;
use crate::utils::password_utils;
use crate::*;

pub async fn signin_account_handler(
    State(AppState { cli, .. }): State<AppState>,
    Json(req): Json<SigninAccountRequest>,
) -> Result<Json<AccountResponse>> {
    debug!("Handling signin request for email: {}", req.email);

    // Find account by email using the GSI
    let (accounts, _bookmark) =
        Account::find_by_email(&cli, &req.email, AccountQueryOption::builder().limit(1)).await?;

    // Check if account exists
    if accounts.is_empty() {
        error!("Account not found for email: {}", req.email);
        return Err(Error::InvalidCredentials);
    }

    let account = &accounts[0];

    // Verify password
    if !password_utils::verify_password(&req.password, &account.password) {
        error!("Invalid password for email: {}", req.email);
        return Err(Error::InvalidCredentials);
    }

    debug!(
        "Successfully authenticated account for email: {}",
        req.email
    );

    Ok(Json(account.to_owned().into()))
}
