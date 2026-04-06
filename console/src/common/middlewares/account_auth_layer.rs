use dioxus::fullstack::axum::{
    extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response,
};

use crate::features::accounts::Account;

pub async fn inject_account(mut req: Request, next: Next) -> Response {
    if req.extensions().get::<Account>().is_some() {
        return next.run(req).await;
    }

    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();

    // Keep bearer precedence: if bearer exists, do not fallback to session in middleware.
    // This preserves current auth semantics where invalid API keys should not be masked by session auth.
    let bearer = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());

    if let Some(token) = bearer {
        if let Ok(account) = crate::common::auth::authenticate_by_credential(&token, cli).await {
            req.extensions_mut().insert(account);
        }
        return next.run(req).await;
    }

    if let Some(session) = req.extensions().get::<tower_sessions::Session>().cloned() {
        if let Ok(account) = crate::common::auth::authenticate_by_session(&session, cli).await {
            req.extensions_mut().insert(account);
        }
    }

    next.run(req).await
}
