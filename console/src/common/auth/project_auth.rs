use dioxus::fullstack::axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use tower_sessions::Session;

use crate::common::{CommonConfig, EntityType, Error, OrganizationRole, Partition};
use crate::features::accounts::{Account, AccountType};
use crate::features::projects::{Project, ProjectError};

use super::{extract_project_id, resolve_account_from_parts};

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
