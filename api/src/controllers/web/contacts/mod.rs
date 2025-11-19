mod submit_contact;

use by_axum::axum::native_routing;

use crate::features::contacts::*;
use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().native_route(
        "/",
        native_routing::post(submit_contact::submit_contact_handler),
    ))
}
