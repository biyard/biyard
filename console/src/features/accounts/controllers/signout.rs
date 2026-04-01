use crate::common::{Deserialize, Result, Serialize};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::Extension;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignoutResponse {
    pub status: String,
}

#[post("/v1/accounts/signout", session: Extension<tower_sessions::Session>)]
pub async fn signout_handler() -> Result<SignoutResponse> {
    let Extension(session) = session;
    session.flush().await?;

    Ok(SignoutResponse {
        status: "OK".to_string(),
    })
}
