use crate::*;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    native_routing::{get, get_service},
};
mod console_handler;

use console_handler::console_handler;
use tmpl_renderer::{IndexTmpl, PageMeta};
use tower_http::services::ServeDir;

pub fn route() -> Result<Router<AppState>> {
    let mut router = Router::new()
        .native_route("/", get(console_handler))
        .fallback(console_handler);

    if config::get().web_build {
        let static_routes = ["/assets", "/favicon.ico", "/tailwind.css", "/main.css"];

        for route in static_routes {
            router = router.nest_service(
                route,
                get_service(ServeDir::new(format!("{}{}", "dist/console", route))),
            );
        }
    }

    Ok(router)
}

pub struct ConsolePage;
impl PageMeta for ConsolePage {
    fn title(&self) -> &'static str {
        "Biyard Console"
    }
    fn description(&self) -> &'static str {
        "Manage your blockchain projects on Biyard"
    }
}

pub type ConsolePageTmpl = (ConsolePage, IndexTmpl);

impl FromRequestParts<AppState> for ConsolePageTmpl {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        let mut path = parts.uri.path().to_string();

        if let Some(q) = parts.uri.query() {
            path = format!("{}?{}", path, q);
        }
        let mut prefix = "";
        if config::get().web_build {
            prefix = "/console/";
        }
        let index_js = config::get().console.index_js;
        let index_css = config::get().console.index_css;
        let host = config::get().domain.to_string();
        let page = ConsolePage;
        let tmpl = IndexTmpl::new(page.title())
            .with_canonical_url(format!("https://{host}{path}"))
            .with_index_js(format!("{}{}", prefix, index_js)) // Deployed /c
            .with_index_css(format!("{}{}", prefix, index_css));
        Ok((page, tmpl))
    }
}
