pub mod create_project;
pub mod delete_project;
pub mod get_project;
pub mod list_projects;
pub mod points;
pub mod tokens;
pub mod update_project;

#[cfg(test)]
mod tests;

use crate::{
    features::{accounts::Account, projects::Project},
    *,
};

pub use create_project::*;
pub use delete_project::*;
pub use get_project::*;
pub use list_projects::*;
pub use update_project::*;

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
) -> std::result::Result<Response<Body>, Error> {
    tracing::debug!("Project authorization middleware");

    // Extract request parts to access headers and URI
    let (mut parts, body) = req.into_parts();

    // Extract authenticated account from request
    let account: &Account = match parts.extensions.get() {
        Some(account) => account,
        _ => {
            // Return 401 Unauthorized if authentication fails
            return Err(Error::Unauthorized);
        }
    };

    // Extract project_id from the URI path
    // Expected format: /v1/projects/:project_id/* or /v1/projects/:project_id
    let path = parts.uri.path();
    let path_segments: Vec<&str> = path.split('/').collect();

    // Find the segment after "projects"
    let project_id = path_segments[1].to_string();

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
            return Err(Error::ProjectNotFound);
        }
        Err(e) => {
            tracing::error!("Failed to fetch project {}: {:?}", project_id, e);
            return Err(Error::Forbidden);
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
        return Err(Error::Forbidden);
    }

    tracing::debug!(
        "Project access authorized for account {:?} on project {}",
        account.pk,
        project_id
    );

    parts.extensions.insert(project);

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
