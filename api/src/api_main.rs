use std::sync::Arc;

use by_axum::axum::Router;

use crate::{features::session::session_manage_layer, *};

pub async fn api_main() -> Result<Router<AppState>> {
    let conf = config::get();

    let app_state = AppState::new(&conf);

    let app = controllers::route(app_state.clone())?;

    Ok(app.layer(session_manage_layer(app_state.cli.clone(), &conf)))
}
