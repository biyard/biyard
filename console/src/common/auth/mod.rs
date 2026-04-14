use dioxus::fullstack::axum::http::request::Parts;
use tower_sessions::Session;
use dioxus::fullstack::axum::extract::FromRequestParts;

use crate::common::Error;
use crate::features::accounts::Account;

mod account_auth;
mod credential_auth;
mod enterprise_auth;
mod project_auth;

pub use enterprise_auth::{EnterpriseContextAuth, SystemAdminAuth};
pub use project_auth::{ProjectAdminAuth, ProjectAuth, ProjectViewerAuth};

pub(crate) use account_auth::{authenticate_by_session, authenticate_by_session_from_parts};
pub(crate) use credential_auth::authenticate_by_credential;

pub(super) fn extract_bearer_token(parts: &Parts) -> Option<String> {
    use dioxus::fullstack::axum::http::header::AUTHORIZATION;
    parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Extract project_id from URL path like /v1/projects/:project_id/...
pub(super) fn extract_project_id(path: &str) -> Option<String> {
    let segments: Vec<&str> = path.split('/').collect();
    for (i, segment) in segments.iter().enumerate() {
        if *segment == "projects" {
            return segments.get(i + 1).map(|s| s.to_string());
        }
    }
    None
}

pub(super) async fn resolve_account_from_parts<S>(
    parts: &mut Parts,
    state: &S,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Account>
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    if let Some(cached) = parts.extensions.get::<Account>() {
        return Ok(cached.clone());
    }

    let account = if let Some(token) = extract_bearer_token(parts) {
        credential_auth::authenticate_by_credential(&token, cli).await?
    } else {
        account_auth::authenticate_by_session_from_parts(parts, state, cli).await?
    };

    parts.extensions.insert(account.clone());
    Ok(account)
}
