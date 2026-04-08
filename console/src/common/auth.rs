use dioxus::fullstack::axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use tower_sessions::Session;

use crate::common::{CommonConfig, EntityType, Error, OrganizationRole, Partition};
use crate::features::accounts::controllers::SESSION_KEY_ACCOUNT_ID;
use crate::features::accounts::{Account, AccountError, AccountType};
use crate::features::credentials::{
    Credential, CredentialError, CredentialQueryOption, CredentialStatus,
};
use crate::features::enterprises::Enterprise;
use crate::features::enterprises::controllers::ensure_current_enterprise_for_account;
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

#[derive(Debug, Clone)]
pub struct SystemAdminAuth {
    pub account: Account,
}

impl<S> FromRequestParts<S> for SystemAdminAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CommonConfig::default();
        let cli = config.dynamodb();

        let account = resolve_account_from_parts(parts, state, cli).await?;
        if account.user_type != AccountType::SystemAdmin {
            return Err(Error::Forbidden);
        }

        Ok(Self { account })
    }
}

#[derive(Debug, Clone)]
pub struct EnterpriseContextAuth {
    pub account: Account,
    pub enterprise: Enterprise,
    pub role: OrganizationRole,
}

impl<S> FromRequestParts<S> for EnterpriseContextAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CommonConfig::default();
        let cli = config.dynamodb();

        let account = resolve_account_from_parts(parts, state, cli).await?;
        let (account, enterprise) = ensure_current_enterprise_for_account(cli, &account).await?;
        let role = account.organization_role;

        Ok(Self {
            account,
            enterprise,
            role,
        })
    }
}

/// Authenticated project context: Bearer + session auth with project ownership verification.
#[derive(Debug, Clone)]
pub struct ProjectAuth {
    pub account: Account,
    pub project: Project,
}

#[derive(Debug, Clone)]
pub struct ProjectViewerAuth {
    pub account: Account,
    pub project: Project,
    pub role: OrganizationRole,
}

#[derive(Debug, Clone)]
pub struct ProjectAdminAuth {
    pub account: Account,
    pub project: Project,
    pub role: OrganizationRole,
}

#[derive(Debug, Clone)]
struct ProjectRoleContext {
    account: Account,
    project: Project,
    role: OrganizationRole,
}

impl<S> FromRequestParts<S> for ProjectAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = extract_project_with_role(parts, state, OrganizationRole::Owner).await?;
        Ok(Self {
            account: ctx.account,
            project: ctx.project,
        })
    }
}

impl<S> FromRequestParts<S> for ProjectViewerAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = extract_project_with_role(parts, state, OrganizationRole::Viewer).await?;
        Ok(Self {
            account: ctx.account,
            project: ctx.project,
            role: ctx.role,
        })
    }
}

impl<S> FromRequestParts<S> for ProjectAdminAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = extract_project_with_role(parts, state, OrganizationRole::Admin).await?;
        Ok(Self {
            account: ctx.account,
            project: ctx.project,
            role: ctx.role,
        })
    }
}

async fn extract_project_with_role<S>(
    parts: &mut Parts,
    state: &S,
    required_role: OrganizationRole,
) -> crate::common::Result<ProjectRoleContext>
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let account = resolve_account_from_parts(parts, state, cli).await?;
    let project = load_project_from_path(parts, cli).await?;
    let role = infer_project_role(cli, &account, &project)
        .await?
        .ok_or(ProjectError::ProjectAccessDenied)?;

    if !role.allows(required_role) {
        return Err(Error::Forbidden);
    }

    Ok(ProjectRoleContext {
        account,
        project,
        role,
    })
}

async fn resolve_account_from_parts<S>(
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
        authenticate_by_credential(&token, cli).await?
    } else {
        authenticate_by_session_from_parts(parts, state, cli).await?
    };

    parts.extensions.insert(account.clone());
    Ok(account)
}

fn extract_bearer_token(parts: &Parts) -> Option<String> {
    parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

async fn load_project_from_path(
    parts: &Parts,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Project> {
    let project_id = extract_project_id(parts.uri.path()).ok_or(ProjectError::ProjectNotFound)?;
    let project_pk = Partition::Project(project_id);
    Project::get(cli, &project_pk, Some(EntityType::Project))
        .await
        .map_err(|e| {
            crate::common::error!("failed to get project from db: {:?}", e);
            Error::from(ProjectError::ProjectNotFound)
        })?
        .ok_or(ProjectError::ProjectNotFound.into())
}

async fn infer_project_role(
    _cli: &aws_sdk_dynamodb::Client,
    account: &Account,
    project: &Project,
) -> crate::common::Result<Option<OrganizationRole>> {
    if account.user_type == AccountType::SystemAdmin {
        return Ok(Some(OrganizationRole::Owner));
    }

    // Single-membership invariant: an Account belongs to at most one
    // Enterprise, and the role on that Enterprise is stored on Account
    // itself (Account.organization_role). A request can access a Project
    // iff the project's organization_id matches the caller's
    // enterprise_id.
    //
    // NOTE: this is the *only* place in the codebase where
    // Account.enterprise_id is compared directly. All other call sites
    // must go through EnterpriseContextAuth so that a future migration
    // to multi-membership only needs to update the auth extractor.
    if matches!(project.organization_id, Partition::None) {
        return Ok(None);
    }

    if matches!(account.enterprise_id, Partition::None) {
        return Ok(None);
    }

    if project.organization_id != account.enterprise_id {
        return Ok(None);
    }

    Ok(Some(account.organization_role))
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

pub(crate) async fn authenticate_by_session_from_parts<S>(
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
    let api_key_hash = crate::common::utils::password_utils::hash_secret_for_lookup(api_key);

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
