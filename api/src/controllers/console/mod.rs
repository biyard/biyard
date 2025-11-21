use crate::*;
use axum::{extract::FromRequestParts, http::request::Parts};
mod console_handler;

use console_handler::console_handler;
use tmpl_renderer::IndexTmpl;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new())
}

impl FromRequestParts<AppState> for IndexTmpl {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        let mut path = parts.uri.path().to_string();

        if let Some(q) = parts.uri.query() {
            path = format!("{}?{}", path, q);
        }
        let host = config::get().domain.to_string();

        Ok(Self::new("Biyard Console").with_canonical_url(format!("https://{host}{path}")))
    }
}
