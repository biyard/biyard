use tower_sessions::Session;

use crate::*;

pub async fn signout_account_handler(
    Extension(session): Extension<Session>,
) -> Result<Json<()>> {
    debug!("Handling signout request");

    // Clear the session
    session.flush().await?;

    debug!("Successfully signed out and cleared session");

    Ok(Json(()))
}
