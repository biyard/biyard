use crate::*;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    native_routing::{get, get_service},
};
mod landing_handler;

use landing_handler::landing_handler;
use tmpl_renderer::{IndexTmpl, PageMeta};
use tower_http::services::ServeDir;

pub fn route() -> Result<Router<AppState>> {
    let path = option_env!("LANDING_PATH").unwrap_or("dist/landing");

    let static_routes = [
        "/assets",
        "/favicon.ico",
        "/logos",
        "/tailwind.css",
        "/main.css",
        "/members",
        "/services",
    ];

    let mut router = Router::new()
        .native_route("/", get(landing_handler))
        .fallback(landing_handler);

    for route in static_routes {
        router = router.nest_service(
            route,
            get_service(ServeDir::new(format!("{}{}", path, route))),
        );
    }

    Ok(router)
}

pub struct LandingPage;
impl PageMeta for LandingPage {
    fn title(&self) -> &'static str {
        "Biyard - Blockchain Launchpad Platform"
    }
    fn description(&self) -> &'static str {
        "Welcome to Biyard - Your Gateway to Blockchain Launchpads"
    }
}

pub type LandingPageTmpl = (LandingPage, IndexTmpl);

impl FromRequestParts<AppState> for LandingPageTmpl {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        let mut path = parts.uri.path().to_string();

        if let Some(q) = parts.uri.query() {
            path = format!("{}?{}", path, q);
        }

        let landing_index_js = option_env!("LANDING_INDEX_JS").unwrap_or("index.js");
        let landing_index_css = option_env!("LANDING_INDEX_CSS").unwrap_or("index.css");
        let host = config::get().domain.to_string();
        let page = LandingPage;
        let tmpl = IndexTmpl::new(page.title())
            .with_canonical_url(format!("https://{host}{path}"))
            .with_index_js(format!("/landing/{}", landing_index_js))
            .with_index_css(format!("/landing/{}", landing_index_css));
        Ok((page, tmpl))
    }
}
