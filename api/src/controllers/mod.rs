use crate::*;

pub mod m1;
pub mod v1;

pub fn route(app_state: AppState) -> Result<Router> {
    Ok(Router::new()
        // v1 is endpoints for users
        .nest("/v1", v1::route()?)
        // m1 is service operation admin endpoints
        .nest("/m1", m1::route()?)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            inject_account_middleware,
        ))
        .with_state(app_state))
}

pub async fn inject_account_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Project authorization middleware");

    // Extract request parts to access headers and URI
    let (mut parts, body) = req.into_parts();

    // Extract authenticated account from request
    let account = match features::accounts::Account::from_request_parts(&mut parts, &state).await {
        Ok(account) => Some(account),
        Err(_) => None,
    };

    if let Some(account) = account {
        parts.extensions.insert(account);
    }

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
