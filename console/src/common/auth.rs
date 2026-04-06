use dioxus::fullstack::axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use tower_sessions::Session;

use crate::common::{CommonConfig, EntityType, Error, Partition};
use crate::features::accounts::controllers::SESSION_KEY_ACCOUNT_ID;
use crate::features::accounts::{Account, AccountError};
use crate::features::credentials::{
    Credential, CredentialError, CredentialQueryOption, CredentialStatus,
};
use crate::features::projects::{Project, ProjectError};

/// Extract Account from request using session only.
impl<S> FromRequestParts<S> for Account
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(account) = parts.extensions.get::<Account>() {
            return Ok(account.clone());
        }

        let config = CommonConfig::default();
        let cli = config.dynamodb();

        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                crate::common::error!("no session found from request: {:?}", e);
                Error::NoSessionFound
            })?;

        let account = authenticate_by_session(&session, cli).await?;
        parts.extensions.insert(account.clone());
        Ok(account)
    }
}

/// Authenticated project context: Bearer + session auth with project ownership verification.
#[derive(Debug, Clone)]
pub struct ProjectAuth {
    pub account: Account,
    pub project: Project,
}

impl<S> FromRequestParts<S> for ProjectAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CommonConfig::default();
        let cli = config.dynamodb();

        // 1. Authenticate: Bearer token first, then session fallback
        let account = if let Some(cached) = parts.extensions.get::<Account>() {
            cached.clone()
        } else {
            let account = if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ").trim();
                        authenticate_by_credential(token, cli).await?
                    } else {
                        authenticate_by_session_from_parts(parts, state, cli).await?
                    }
                } else {
                    authenticate_by_session_from_parts(parts, state, cli).await?
                }
            } else {
                authenticate_by_session_from_parts(parts, state, cli).await?
            };
            parts.extensions.insert(account.clone());
            account
        };

        // 2. Extract project_id from path
        let path = parts.uri.path();
        let project_id = extract_project_id(path).ok_or(ProjectError::ProjectNotFound)?;

        // 3. Get project and verify ownership
        let project_pk = Partition::Project(project_id);
        let project = Project::get(cli, &project_pk, Some(EntityType::Project))
            .await
            .map_err(|e| {
                crate::common::error!("failed to get project from db: {:?}", e);
                Error::from(ProjectError::ProjectNotFound)
            })?
            .ok_or(ProjectError::ProjectNotFound)?;

        project.verify_ownership(&account.pk)?;

        Ok(ProjectAuth { account, project })
    }
}

/// Extract project_id from URL path like /v1/projects/:project_id/...
fn extract_project_id(path: &str) -> Option<String> {
    let segments: Vec<&str> = path.split('/').collect();
    for (i, segment) in segments.iter().enumerate() {
        if *segment == "projects" {
            return segments.get(i + 1).map(|s| s.to_string());
        }
    }
    None
}

async fn authenticate_by_session_from_parts<S>(
    parts: &mut Parts,
    state: &S,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Account>
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    let session = Session::from_request_parts(parts, state)
        .await
        .map_err(|e| {
            crate::common::error!("no session found from request: {:?}", e);
            Error::NoSessionFound
        })?;

    authenticate_by_session(&session, cli).await
}

pub(crate) async fn authenticate_by_session(
    session: &Session,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Account> {
    let account_pk: String = session
        .get(SESSION_KEY_ACCOUNT_ID)
        .await
        .map_err(|e| {
            crate::common::error!("no account id found from session: {:?}", e);
            Error::NoSessionFound
        })?
        .ok_or(Error::NoSessionFound)?;

    let partition: Partition = account_pk.parse().map_err(|_| Error::NoSessionFound)?;

    let account = Account::get(cli, &partition, Some(EntityType::Account))
        .await
        .map_err(|e| {
            crate::common::error!("failed to get account from db: {:?}", e);
            Error::NoSessionFound
        })?;

    match account {
        Some(acc) => Ok(acc),
        None => {
            let _ = session.flush().await;
            Err(AccountError::AccountNotFound.into())
        }
    }
}

pub(crate) async fn authenticate_by_credential(
    api_key: &str,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Account> {
    let api_key_hash = crate::common::utils::password_utils::hash_password(api_key);

    let (credentials, _) = Credential::find_by_api_key_hash(
        cli,
        &api_key_hash,
        CredentialQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| {
        crate::common::error!("failed to query credential by api key: {:?}", e);
        Error::from(CredentialError::InvalidApiKey)
    })?;

    if credentials.is_empty() {
        return Err(CredentialError::InvalidApiKey.into());
    }

    let credential = &credentials[0];

    if credential.status != CredentialStatus::Active {
        return Err(CredentialError::InvalidApiKey.into());
    }

    // Update last_used_at (fire-and-forget)
    let _ = Credential::updater(credential.pk.clone(), credential.sk.clone())
        .with_last_used_at(crate::common::utils::time_utils::get_now())
        .execute(cli)
        .await;

    // Get the account
    let account = Account::get(cli, &credential.account_id, Some(EntityType::Account))
        .await?
        .ok_or(AccountError::AccountNotFound)?;

    Ok(account)
}
