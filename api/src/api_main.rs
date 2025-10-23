use std::sync::Arc;

use by_axum::axum::Router;

use crate::*;

pub async fn api_main() -> Result<Router<AppState>> {
    let conf = config::get();

    let app_state = AppState::new(&conf);

    let app = controllers::route(app_state)?;

    Ok(app)
}
