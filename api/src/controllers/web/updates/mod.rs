mod follow_updates;

use by_axum::axum::native_routing;
use follow_updates::follow_updates_handler;

use crate::features::contacts::*;
use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().native_route("/", native_routing::post(follow_updates_handler)))
}
