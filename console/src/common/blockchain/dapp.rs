use dioxus::fullstack::axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

const EXCHANGE_HTML: &str = include_str!("../../../dapp/exchange.html");
const BUYBACK_HTML: &str = include_str!("../../../dapp/buyback.html");

pub fn router() -> Router {
    Router::new()
        .route("/dapp/exchange", get(serve_exchange))
        .route("/dapp/buyback", get(serve_buyback))
}

async fn serve_exchange() -> impl IntoResponse {
    Html(EXCHANGE_HTML)
}

async fn serve_buyback() -> impl IntoResponse {
    Html(BUYBACK_HTML)
}
