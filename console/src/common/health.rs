use dioxus::fullstack::axum::{
    Json, Router,
    response::IntoResponse,
    routing::get,
};
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/v1/health", get(health))
}

async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}
