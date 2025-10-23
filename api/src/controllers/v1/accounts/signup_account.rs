use tower_sessions::Session;

use crate::features::accounts::*;
use crate::features::session::SESSION_KEY_ACCOUNT_ID;
use crate::utils::password_utils;
use crate::*;

pub async fn signup_account_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<SignupAccountRequest>,
) -> Result<Json<AccountResponse>> {
    info!("Handling signup request for email: {}", req.email);

    let (accounts, _bookmark) =
        Account::find_by_email(&cli, &req.email, AccountQueryOption::builder().limit(1)).await?;

    info!("Checked existing accounts for email: {:?}", accounts);

    if accounts.len() > 0 {
        return Err(Error::EmailAlreadyExists);
    }

    // Hash the password (note: hashed_password field actually contains plain password)
    let hashed_password = password_utils::hash_password(&req.hashed_password);
    info!("Password hashed successfully for email: {}", req.email);

    // Create a new account with hashed password
    let account = Account::new(req.name.clone(), req.email.clone(), hashed_password);

    // Save to DynamoDB
    account.create(&cli).await?;

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    Ok(Json(account.into()))
}
