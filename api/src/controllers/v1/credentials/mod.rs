use by_axum::axum::{routing::{delete, get, post}, Router};

use crate::*;

mod create_credential;
mod list_credentials;
mod revoke_credential;

#[cfg(test)]
mod tests;

use create_credential::*;
use list_credentials::*;
use revoke_credential::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", post(create_credential_handler))
        .route("/", get(list_credentials_handler))
        .route("/:credential_id", delete(revoke_credential_handler)))
}
