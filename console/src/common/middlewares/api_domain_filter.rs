use dioxus::fullstack::axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn filter_api_domain(req: Request, next: Next) -> Response {
    let Some(api_domain) = option_env!("API_DOMAIN") else {
        return next.run(req).await;
    };

    let host = req
        .headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let host = host.split(':').next().unwrap_or(host);

    if host == api_domain {
        let path = req.uri().path();
        if path.starts_with("/v1/") || path.starts_with("/m1/") {
            return next.run(req).await;
        }
        return StatusCode::NOT_FOUND.into_response();
    }

    next.run(req).await
}
