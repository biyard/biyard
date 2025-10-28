use tower_sessions::Session;

use crate::features::accounts::*;
use crate::*;

pub async fn get_current_account_handler(
    State(state): State<AppState>,
    Extension(session): Extension<Session>,
) -> Result<Json<AccountResponse>> {
    debug!("Handling get current account request");

    // Get account from session
    let account = Account::from_session(session, &state).await?;

    debug!("Successfully retrieved current account: {}", account.pk);

    Ok(Json(account.into()))
}
