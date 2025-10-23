use std::sync::Arc;

use by_axum::axum::Router;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::{features::session::session_manage_layer, *};

pub async fn api_main() -> Result<Router> {
    let conf = config::get();
    let app = by_axum::new();
    let app_state = AppState::new(&conf);
    let router = controllers::route(app_state.clone())?;

    let app = app
        .merge(router)
        .layer(session_manage_layer(app_state.cli.clone(), &conf))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::span!(
                        Level::INFO,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        headers = ?request.headers(),
                        version = ?request.version()
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        if !response.status().is_success() {
                            tracing::error!(
                                status = %response.status(),
                                latency = ?latency,
                                "error response generated"
                            );
                            return;
                        }

                        tracing::info!(
                            status = %response.status(),
                            latency = ?latency,
                            "response generated"
                        )
                    },
                ),
        );

    Ok(app)
}
