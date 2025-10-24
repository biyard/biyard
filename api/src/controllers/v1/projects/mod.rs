pub mod create_project;
pub mod delete_project;
pub mod get_project;
pub mod list_projects;
pub mod points;
pub mod tokens;
pub mod update_project;

#[cfg(test)]
mod tests;

use by_axum::axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
};

use crate::{features::projects::Project, *};

pub use create_project::*;
pub use delete_project::*;
pub use get_project::*;
pub use list_projects::*;
pub use update_project::*;

/// Helper function to fetch project and verify ownership
///
/// This utility function:
/// 1. Fetches the project from DynamoDB by ID
/// 2. Verifies that the account owns the project
/// 3. Returns the project if successful
/// 4. Returns 404 if project not found, 403 if access denied
///
/// # Usage in handlers
/// ```rust
/// let project = verify_and_get_project(&state.cli, &project_id, &account).await?;
/// ```
pub async fn verify_and_get_project(
    ddb_client: &aws_sdk_dynamodb::Client,
    project_id: &str,
    account: &features::accounts::Account,
) -> Result<Project> {
    tracing::debug!(
        "Verifying project ownership for project_id: {} by account: {:?}",
        project_id,
        account.pk
    );

    // Get the project from DynamoDB
    let project_pk = Partition::Project(project_id.to_string());
    let project = Project::get(ddb_client, project_pk, Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(account)?;

    tracing::debug!(
        "Project ownership verified successfully for project: {}",
        project_id
    );

    Ok(project)
}

pub fn route() -> Result<Router<AppState>> {
    let conf = config::get();

    let app_state = AppState::new(&conf);

    Ok(Router::new()
        .route(
            "/:project_id",
            get(get_project_handler)
                .put(update_project_handler)
                .delete(delete_project_handler),
        )
        .nest("/:project_id/points", points::route()?)
        .nest("/:project_id/tokens", tokens::route()?)
        .layer(middleware::from_fn_with_state(
            app_state,
            authorize_project_permission,
        ))
        .route("/", get(list_projects_handler).post(create_project_handler)))
}

/// Middleware to authorize project access by verifying ownership
///
/// This middleware:
/// 1. Extracts the authenticated account from the request
/// 2. Extracts the project_id from the path parameter
/// 3. Fetches the project from DynamoDB
/// 4. Verifies that the account owns the project
/// 5. Returns 401 if not authenticated, 404 if project not found, 403 if access denied
pub async fn authorize_project_permission(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Project authorization middleware");

    // Extract request parts to access headers and URI
    let (mut parts, body) = req.into_parts();

    // Extract authenticated account from request
    let account = match features::accounts::Account::from_request_parts(&mut parts, &state).await {
        Ok(account) => account,
        Err(e) => {
            tracing::warn!("Authentication failed in project authorization: {:?}", e);
            // Return 401 Unauthorized if authentication fails
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Extract project_id from the URI path
    // Expected format: /v1/projects/:project_id/* or /v1/projects/:project_id
    let path = parts.uri.path();
    let path_segments: Vec<&str> = path.split('/').collect();

    // Find the segment after "projects"
    let project_id = path_segments
        .iter()
        .position(|&seg| seg == "projects")
        .and_then(|idx| path_segments.get(idx + 1))
        .filter(|&&seg| !seg.is_empty())
        .map(|&seg| seg.to_string());

    let project_id = match project_id {
        Some(id) => id,
        None => {
            tracing::debug!("No project_id found in path, allowing request to proceed");
            // No project_id in path (e.g., /v1/projects/), allow request to proceed
            let req = Request::from_parts(parts, body);
            return Ok(next.run(req).await);
        }
    };

    tracing::debug!(
        "Verifying project access for project_id: {} by account: {:?}",
        project_id,
        account.pk
    );

    // Get the project from DynamoDB
    let project_pk = Partition::Project(project_id.clone());
    let project = match Project::get(&state.cli, project_pk, Some(EntityType::Project)).await {
        Ok(Some(project)) => project,
        Ok(None) => {
            tracing::warn!("Project not found: {}", project_id);
            return Err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            tracing::error!("Failed to fetch project {}: {:?}", project_id, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Verify ownership
    if let Err(e) = project.verify_ownership(&account) {
        tracing::warn!(
            "Project access denied for account {:?} on project {}: {:?}",
            account.pk,
            project_id,
            e
        );
        return Err(StatusCode::UNAUTHORIZED);
    }

    tracing::debug!(
        "Project access authorized for account {:?} on project {}",
        account.pk,
        project_id
    );

    parts.extensions.insert(account);
    parts.extensions.insert(project);

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
