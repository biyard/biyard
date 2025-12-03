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
    // LANDING_FILE_PATH is set in build.rs
    let path = option_env!("LANDING_FILE_PATH").unwrap_or("dist/landing");

    let static_routes = ["/assets", "/favicon.ico", "/tailwind.css", "/main.css"];

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

        let base_path = config::get().landing.base_path;
        let index_js = config::get().landing.index_js;
        let index_css = config::get().landing.index_css;
        let host = config::get().domain.to_string();
        let page = LandingPage;
        let tmpl = IndexTmpl::new(page.title())
            .with_canonical_url(format!("https://{host}{path}"))
            .with_index_js(format!("{}/{}", base_path, index_js))
            .with_index_css(format!("{}/{}", base_path, index_css));
        Ok((page, tmpl))
    }
}
